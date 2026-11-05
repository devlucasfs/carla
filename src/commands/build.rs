use std::{
    env,
    ffi::{CString, c_char},
    fs,
};

use colorize::AnsiColor;

use crate::{
    cbinds, cli::{Build::{Auto, Pattern}, BuildPattern, CLI, Command}, compiler::{charsert, codegen::Codegen, nesting, scanner::Scanner, symbols::Symbols, visitor}, utils::{compiler_outputs::CompilerOutputs, sys, uuid},
};

unsafe extern "C" {
    fn __morgana_build(
        path: *mut c_char,
        target: *mut c_char,
        extensor: *mut c_char,
        libs: *mut c_char,
        sake: *mut c_char,
        enable_then: bool,
        use_ram: bool
    );
}

impl Command {
    pub fn build(&self, c: &CLI) {
        if c.use_ram {
            CompilerOutputs::log(
                format!("Using ram to improve Morgana comptime")
            );
        }

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

        let sake = current_dir.join("Sake");
        if !sake.exists() {
            CompilerOutputs::fatal_internal("Can't reach Samar eva's addapter file: That file is needed because into it has all compilation rules you are using for your peoject.");
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

        println!("{} Starting compilation proccess.", "1/2".yellow().bold());

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

        // println!("{scan}");

        let mut scanner = Scanner::new(scan);
        scanner.read();

        let mut symbols = Symbols::new();
        symbols.push_scope();
        charsert::append(&mut symbols);

        cbinds::Runa::init(&data.constexpr);

        println!("\u{001B}[1A{} Starting compilation proccess.", "2/2".yellow().bold());
        println!("{} Building for {} target(s)", "└─".b_grey(), data.builds.len().to_string().b_yellow());

        let contexts = nesting::work(scanner.tokens);
        for build in &data.builds {
            symbols.push_scope();
            let arch_os = match build {
                Pattern(x) => {
                    let BuildPattern {
                        arch, os
                    } = x;

                    cbinds::Runa::starts(format!("local arch, os = '{arch}', '{}'", if os.is_empty() {
                        String::from("none")
                    } else { os.clone() }));

                    x.to_string()
                }
                Auto => {
                    let (arch, os) = sys::get(build);

                    cbinds::Runa::starts(
                        format!("local arch, os = '{arch}', '{}'", if os.is_empty() {
                            String::from("none")
                        } else { os.clone() })
                    );

                    format!("{arch}-{os}")
                }
            };

            print!("{}{arch_os}", " # ".b_yellow());

            let nodes = visitor::work(&contexts, &mut symbols);
            // let libs: Vec<String> = vec!["nashj".into(), "rmem".into(), "routines".into()];
            let libs: Vec<String> = vec![];
            let code = Codegen::new(nodes.clone()).generate(&mut symbols, None);
            // println!("\n-------------------- OUTPUT:\n\n{code}");
            let temp_file = format!("out.{}.morg", uuid::gen_hex(265));
            let morgana_temp = env::temp_dir().join(temp_file);

            if let Err(_) = fs::write(&morgana_temp, code) {
                CompilerOutputs::fatal("Can't write temporary Morgana file.");
            }

            let path = CString::new(morgana_temp.to_str().unwrap()).unwrap();
            let target = CString::new(target.to_str().unwrap()).unwrap();
            let sake = CString::new(sake.to_str().unwrap()).unwrap();
            let extensor = CString::new(arch_os.clone()).unwrap();
            let libs = CString::new(libs.join(",")).unwrap();

            unsafe {
                __morgana_build(
                    path.as_ptr() as *mut c_char,
                    target.as_ptr() as *mut c_char,
                    extensor.as_ptr() as *mut c_char,
                    libs.as_ptr() as *mut c_char,
                    sake.as_ptr() as *mut c_char,
                    c.use_ram,
                    c.enable_then
                );
            }

            let _ = fs::remove_file(&temp);
            let _ = fs::remove_file(morgana_temp);
            symbols.pop_scope();

            println!("\r{}{arch_os} {}", " ✔️ ".b_green(), "compiled.".b_green());
        }

        cbinds::Runa::deinit();
    }
}
