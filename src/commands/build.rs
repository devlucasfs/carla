use std::{
    env,
    ffi::{CString, c_char},
    fs,
};

use crate::{
    cbinds,
    cli::{CLI, Command},
    compiler::{charsert, codegen::Codegen, nesting, scanner::Scanner, symbols::Symbols, visitor},
    utils::{compiler_outputs::CompilerOutputs, sys, uuid},
};

unsafe extern "C" {
    fn __morgana_build(
        path: *mut c_char,
        target: *mut c_char,
        extensor: *mut c_char,
        libs: *mut c_char,
    );
}

impl Command {
    pub fn build(&self, _: &CLI) {
        let Ok(current_dir) = env::current_dir() else {
            CompilerOutputs::fatal("Can't read the current directory.");
            unreachable!();
        };

        let target = current_dir.join("target");
        if !target.exists() {
            if let Err(_) = fs::create_dir_all(&target) {
                CompilerOutputs::fatal_internal("Can't create target folder.");
            }
        }

        let core = current_dir.join("core");
        if !core.exists() {
            if let Err(_) = fs::create_dir_all(&core) {
                CompilerOutputs::fatal_internal("Can't create core folder.");
            }
        }

        let Command::Build(data) = self else {
            unreachable!()
        };

        let temp_file = format!("output.{}.crl.e", uuid::gen_hex(265));
        let temp = env::temp_dir().join(temp_file);

        let status = std::process::Command::new("norn")
            .arg(&data.main)
            .arg(&temp)
            .status();

        match status {
            Ok(status) if status.success() => {}
            _ => CompilerOutputs::fatal("Norn precompiler failed."),
        }

        let Ok(scan) = fs::read_to_string(&temp) else {
            CompilerOutputs::fatal("Can't read the precompiled main file.");
            unreachable!()
        };

        println!("{scan}");

        let mut scanner = Scanner::new(scan);
        scanner.read();

        let mut symbols = Symbols::new();
        symbols.push_scope();
        charsert::append(&mut symbols);

        cbinds::Runa::init(&data.constexpr);

        let contexts = nesting::work(scanner.tokens);
        for build in &data.builds {
            let (arch, os) = sys::get(build);

            cbinds::Runa::starts(format!("local arch, os = '{arch}', '{os}'"));

            let nodes = visitor::work(&contexts, &mut symbols);
            let libs: Vec<String> = vec!["nashj".into(), "rmem".into(), "routines".into()];
            let code = Codegen::new(nodes.clone()).generate(&mut symbols, None);
            println!("\n-------------------- OUTPUT:\n\n{code}");
            let temp_file = format!("out.{}.morg", uuid::gen_hex(265));
            let morgana_temp = env::temp_dir().join(temp_file);

            if let Err(_) = fs::write(&morgana_temp, code) {
                CompilerOutputs::fatal("Can't write temporary Morgana file.");
            }

            let path = CString::new(morgana_temp.to_str().unwrap()).unwrap();
            let target = CString::new(target.to_str().unwrap()).unwrap();
            let extensor = CString::new("").unwrap();
            let libs = CString::new(libs.join(",")).unwrap();

            unsafe {
                __morgana_build(
                    path.as_ptr() as *mut c_char,
                    target.as_ptr() as *mut c_char,
                    extensor.as_ptr() as *mut c_char,
                    libs.as_ptr() as *mut c_char,
                );
            }

            let _ = fs::remove_file(&temp);
            let _ = fs::remove_file(morgana_temp);
        }

        cbinds::Runa::deinit();
    }
}
