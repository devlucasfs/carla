use std::{
    env,
    ffi::{CString, c_char},
    fs,
    io::{self, Write},
    process::Command as ProcessCommand,
    sync::mpsc,
    thread,
};

use colorize::AnsiColor;

use crate::{
    cbinds,
    cli::{Build::{Auto, Pattern}, BuildPattern, CLI, Command},
    compiler::{
        charsert,
        codegen::Codegen,
        nesting,
        scanner::Scanner,
        symbols::Symbols,
        visitor,
    },
    utils::{
        compiler_outputs::CompilerOutputs,
        sys,
        uuid,
    },
};

unsafe extern "C" {
    fn __morgana_build(
        path: *mut c_char,
        target: *mut c_char,
        extensor: *mut c_char,
        libs: *mut c_char,
        sake: *mut c_char,
        enable_then: bool,
        use_ram: bool,
    );
}

struct BuildResult {
    owner: String,
    success: bool,
    status: Option<i32>,
    stdout: String,
    stderr: String,
}

pub fn morgana_worker_entry() -> bool {
    let args: Vec<String> = env::args().collect();

    if args.get(1).map(String::as_str) != Some("--morgana-worker") {
        return false;
    }

    if args.len() < 9 {
        eprintln!("Invalid Morgana worker arguments.");
        std::process::exit(2);
    }

    let path = CString::new(args[2].as_str()).unwrap();
    let target = CString::new(args[3].as_str()).unwrap();
    let extensor = CString::new(args[4].as_str()).unwrap();
    let libs = CString::new(args[5].as_str()).unwrap();
    let sake = CString::new(args[6].as_str()).unwrap();

    let enable_then = args[7] == "1";
    let use_ram = args[8] == "1";

    unsafe {
        __morgana_build(
            path.as_ptr() as *mut c_char,
            target.as_ptr() as *mut c_char,
            extensor.as_ptr() as *mut c_char,
            libs.as_ptr() as *mut c_char,
            sake.as_ptr() as *mut c_char,
            enable_then,
            use_ram,
        );
    }

    std::process::exit(0);
}

impl Command {
    pub fn build(&self, c: &CLI) {
        if c.use_ram {
            CompilerOutputs::log("Using ram to improve Morgana comptime");
        }

        let Ok(current_dir) = env::current_dir() else {
            CompilerOutputs::fatal("Can't read the current directory.");
            unreachable!();
        };

        let target = current_dir.join("target");

        if !target.exists() {
            if fs::create_dir_all(&target).is_err() {
                CompilerOutputs::fatal_internal("Can't create target folder.");
            }
        }

        let sake = current_dir.join("Sake");

        if !sake.exists() {
            CompilerOutputs::fatal_internal(
                "Can't reach Samar eva's addapter file: That file is needed because into it has all compilation rules you are using for your peoject."
            );
        }

        let core = current_dir.join("core");

        if !core.exists() {
            if fs::create_dir_all(&core).is_err() {
                CompilerOutputs::fatal_internal("Can't create core folder.");
            }
        }

        let Command::Build(data) = self else {
            unreachable!()
        };

        let temp_file = format!("output.{}.crl.e", uuid::gen_hex(265));
        let temp = env::temp_dir().join(temp_file);

        println!(
            "{} Starting compilation proccess.",
            "1/2".yellow().bold()
        );

        let status = ProcessCommand::new("norn")
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

        let _ = fs::remove_file(&temp);

        let mut scanner = Scanner::new(scan);
        scanner.read();

        cbinds::Runa::init(&data.constexpr);

        println!(
            "\u{001B}[1A{} Starting compilation proccess.",
            "2/2".yellow().bold()
        );

        println!(
            "{} Building for {} target(s)",
            "└─".b_grey(),
            data.builds.len().to_string().b_yellow()
        );

        let contexts = nesting::work(scanner.tokens);

        let executable = match env::current_exe() {
            Ok(path) => path,
            Err(_) => {
                CompilerOutputs::fatal("Can't locate compiler executable.");
                unreachable!();
            }
        };

        let total = data.builds.len();

        let (tx, rx) = mpsc::channel::<(usize, BuildResult)>();

        let mut threads = Vec::new();
        let mut owners = Vec::new();

        for (index, build) in data.builds.iter().enumerate() {
            let mut symbols = Symbols::new();
            symbols.push_scope();
            charsert::append(&mut symbols);

            let arch_os = match build {
                Pattern(x) => {
                    let BuildPattern { arch, os } = x;

                    cbinds::Runa::starts(format!(
                        "local arch, os = '{arch}', '{}'",
                        if os.is_empty() {
                            String::from("none")
                        } else {
                            os.clone()
                        }
                    ));

                    x.to_string()
                }

                Auto => {
                    let (arch, os) = sys::get(build);

                    cbinds::Runa::starts(format!(
                        "local arch, os = '{arch}', '{}'",
                        if os.is_empty() {
                            String::from("none")
                        } else {
                            os.clone()
                        }
                    ));

                    format!("{arch}-{os}")
                }
            };

            let build_target = target.join(&arch_os);

            if fs::create_dir_all(&build_target).is_err() {
                CompilerOutputs::fatal_internal(
                    "Can't create architecture target folder."
                );
            }

            let nodes = visitor::work(&contexts, &mut symbols);
            let libs: Vec<String> = vec![];

            let code = Codegen::new(nodes).generate(&mut symbols, None);

            let temp_file = format!("out.{}.morg", uuid::gen_hex(265));
            let morgana_temp = env::temp_dir().join(temp_file);

            if fs::write(&morgana_temp, code).is_err() {
                CompilerOutputs::fatal(
                    "Can't write temporary Morgana file."
                );
            }

            let owner = arch_os.clone();
            let worker = executable.clone();
            let build_target = build_target.clone();
            let sake = sake.clone();

            let use_ram = c.use_ram;
            let enable_then = c.enable_then;
            let libs = libs.join(",");

            owners.push(owner.clone());

            println!("{} {}", "#".yellow().bold(), owner);
            let _ = io::stdout().flush();

            let tx = tx.clone();

            threads.push(thread::spawn(move || {
                let output = ProcessCommand::new(worker)
                    .arg("--morgana-worker")
                    .arg(&morgana_temp)
                    .arg(&build_target)
                    .arg(&owner)
                    .arg(&libs)
                    .arg(&sake)
                    .arg(if enable_then { "1" } else { "0" })
                    .arg(if use_ram { "1" } else { "0" })
                    .output();

                let _ = fs::remove_file(&morgana_temp);

                let result = match output {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(
                            &output.stdout
                        ).into_owned();

                        let stderr = String::from_utf8_lossy(
                            &output.stderr
                        ).into_owned();

                        let status = output.status.code();
                        let success = status == Some(0) &&  stderr.is_empty();

                        BuildResult {
                            owner,
                            success,
                            status,
                            stdout,
                            stderr,
                        }
                    }

                    Err(error) => BuildResult {
                        owner,
                        success: false,
                        status: None,
                        stdout: String::new(),
                        stderr: format!(
                            "Failed to start Morgana worker: {error}"
                        ),
                    },
                };

                let _ = tx.send((index, result));
            }));
        }

        drop(tx);

        let mut completed = vec![false; total];
        let mut failed = Vec::new();

        while let Ok((index, result)) = rx.recv() {
            completed[index] = true;

            print!("\x1b[s");
            print!("\x1b[{}A", total - index);
            print!("\r\x1b[2K");

            if result.success {
                print!(
                    "{} {}",
                    "✔️".b_green(),
                    result.owner
                );
            } else {
                print!(
                    "{} {}",
                    "x".b_red(),
                    result.owner
                );
            }

            print!("\x1b[u");

            let _ = io::stdout().flush();

            if !result.success {
                failed.push(result);
            }
        }

        for handle in threads {
            let _ = handle.join();
        }

        for (index, owner) in owners.into_iter().enumerate() {
            if !completed[index] {
                print!("\x1b[s");
                print!("\x1b[{}A", total - index);
                print!("\r\x1b[2K");
                print!("{} {}", "x".b_red(), owner);
                print!("\x1b[u");

                let _ = io::stdout().flush();

                failed.push(BuildResult {
                    owner,
                    success: false,
                    status: None,
                    stdout: String::new(),
                    stderr: "Build thread panicked.".to_string(),
                });
            }
        }

        cbinds::Runa::deinit();

        if !failed.is_empty() {
            println!();

            for error in &failed {
                println!(
                    "\n{}",
                    ["Comptime error when build ", &error.owner]
                        .concat()
                        .yellow()
                        .bold()
                );

                if error.stdout.is_empty() && error.stderr.is_empty() {
                    println!(
                        "{}",
                        "Morgana terminated without output.".grey().bold()
                    );
                    continue;
                }

                if error.status.unwrap_or(14) == 14
                    && !error.stdout.is_empty()
                {
                    println!(
                        "{}: {}",
                        "stdout".grey().bold(),
                        error.stdout
                    );
                }

                if !error.stderr.is_empty() {
                    println!(
                        "{}: {}",
                        "stderr".grey().bold(),
                        error.stderr
                    );
                }
            }

            println!();
            CompilerOutputs::fatal("One or more builds failed.");
        }
    }
}
