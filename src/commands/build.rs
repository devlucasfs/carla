#[allow(unused_imports)]
use crate::{command, command_fn, terminal::Command, versions::FIRST_RELEASE};

use std::{
    env, ffi::{CString, c_char}, fs, process::Command as ProcessCommand, sync::mpsc, thread, time::Instant, io::{self, Write},
};

use colorize::AnsiColor;

use crate::{
    cbinds::Runa, compiler::{charsert, codegen::Codegen, nesting, scanner::Scanner, symbols::Symbols, visitor}, utils::{
        compiler_outputs::CompilerOutputs, eva::{Eva, EvaValue}, sys, uuid,
    },
};

unsafe extern "C" {
    fn __morgana_build(
        path: *mut c_char,
        target: *mut c_char,
        extensor: *mut c_char,
        sake: *mut c_char,
        extra: *mut c_char,
    );
}

struct BuildResult {
    owner: String,
    success: bool,
    status: Option<i32>,
    stdout: String,
    stderr: String,
    elapsed: f64,
}

pub fn morgana_worker_entry() -> bool {
    let args: Vec<String> = env::args().collect();

    if args.get(1).map(String::as_str) != Some("--morgana-worker") {
        return false;
    }

    if args.len() < 7 {
        eprintln!("Invalid Morgana worker arguments.");
        std::process::exit(2);
    }

    let path = CString::new(args[2].as_str()).unwrap();
    let target = CString::new(args[3].as_str()).unwrap();
    let extensor = CString::new(args[4].as_str()).unwrap();
    let sake = CString::new(args[5].as_str()).unwrap();
    let extra = CString::new(args[6].as_str()).unwrap();

    unsafe {
        __morgana_build(
            path.as_ptr() as *mut c_char,
            target.as_ptr() as *mut c_char,
            extensor.as_ptr() as *mut c_char,
            sake.as_ptr() as *mut c_char,
            extra.as_ptr() as *mut c_char
        );
    }

    std::process::exit(0);
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BuildPattern {
    pub arch: String,
    pub os: String,
}

impl BuildPattern {
    pub fn to_string(&self) -> String {
        if self.os.is_empty() {
            self.arch.clone()
        } else {
            format!("{}-{}", self.arch, self.os)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Build {
    Pattern(BuildPattern),
    Auto,
}

fn pattern_from_string(s: String) -> Build {
    if s.contains('-') {
        let (arch, os) = s.rsplit_once('-').unwrap();
        return Build::Pattern(BuildPattern {
            arch: arch.into(),
            os: os.into(),
        });
    }

    Build::Pattern(BuildPattern {
        arch: s, ..Default::default()
    })
}

pub fn build_patterns(eva: &Eva) -> Result<Vec<Build>, String> {
    let EvaValue::List(builds) = eva.get("target", "builds") else {
        return Err("The `builds` field in the target.eva file need to be a list.".into());
    };

    let mut patterns = Vec::new();

    for i in 0..builds.len() {
        match builds.get(i) {
            EvaValue::String(s) if s.to_lowercase() == "auto" => patterns.push(Build::Auto),
            EvaValue::String(s) => patterns.push(pattern_from_string(s)),
            EvaValue::Map(map) => {
                let EvaValue::String(os) = map.get("os") else {
                    return Err(
                        "The `os` field in the build pattern map need to be a string.".into(),
                    );
                };

                let EvaValue::String(arch) = map.get("arch") else {
                    return Err(
                        "The `arch` field in the build pattern map need to be a string.".into(),
                    );
                };

                patterns.push(Build::Pattern(BuildPattern { arch, os }));
            }

            _ => return Err(format!("the {}th argument its invalid.", i + 1)),
        }
    }

    Ok(patterns)
}

command_fn!(build(args) {
    mcu:       String    = "extra-mcu",
    target:    String    = "target",
    constexpr: String    = "constexpr",
    entry:     String    = "main",
    @bool morgana_bypass = "mw",
} => {
    let Ok(current_dir) = env::current_dir() else {
        CompilerOutputs::fatal("Can't read the current directory.");
        unreachable!();
    };

    let target_dir = current_dir.join("target");

    if !target_dir.exists() {
        if fs::create_dir_all(&target_dir).is_err() {
            CompilerOutputs::fatal_internal("Can't create target folder.");
        }
    }

    let sake = current_dir.join("Sake");

    if !sake.exists() {
        CompilerOutputs::fatal_internal(
            "Can't reach Samar Eva's addapter file: That file is needed because into it has all compilation rules you are using for your peoject."
        );
    }

    let core = current_dir.join("core");

    if !core.exists() {
        if fs::create_dir_all(&core).is_err() {
            CompilerOutputs::fatal_internal("Can't create core folder.");
        }
    }

    let temp_file = format!("output.{}.crl.e", uuid::gen_hex(265));
    let temp = env::temp_dir().join(temp_file);

    println!(
        "{} Starting compilation proccess.",
        "1/2".yellow().bold()
    );

    let Some(main) = entry else {
        CompilerOutputs::fatal("Unknown entry file");
        unreachable!()
    };

    let status = ProcessCommand::new("norn")
        .arg(&main)
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

    let Some(defines) = constexpr else {
        CompilerOutputs::fatal("Unknown constexpr (defines) file");
        unreachable!()
    };

    let _ = fs::remove_file(&temp);

    let mut scanner = Scanner::new(scan);
    scanner.read();
    Runa::init(defines);

    println!(
        "\u{001B}[1A{} Starting compilation proccess.",
        "2/2".yellow().bold()
    );

    let eva_f = current_dir.join("target.eva");
    let eva = &Eva::open(&eva_f.to_string_lossy());

    let mut builds = match build_patterns(eva) {
        Ok(x) => x,
        Err(e) => {
            CompilerOutputs::fatal(e);
            unreachable!()
        }
    };

    if let Some(target) = target {
        builds.clear();
        builds.push(pattern_from_string(target));
    }

    println!(
        "{} Building for {} target(s)",
        "└─".b_grey(),
        builds.len().to_string().b_yellow()
    );

    let contexts = nesting::work(scanner.tokens);

    let executable = match env::current_exe() {
        Ok(path) => path,
        Err(_) => {
            CompilerOutputs::fatal("Can't locate compiler executable.");
            unreachable!();
        }
    };

    let total_start = Instant::now();
    let total = builds.len();

    let (tx, rx) = mpsc::channel::<(usize, BuildResult)>();

    let mut threads = Vec::new();
    let mut owners = Vec::new();

    for (index, build) in builds.iter().enumerate() {
        let mut symbols = Symbols::new();
        symbols.push_scope();
        charsert::append(&mut symbols);

        let arch_os = match build {
            Build::Pattern(x) => {
                let BuildPattern { arch, os } = x;

                Runa::starts(format!(
                    "local arch, os = '{arch}', '{}'",
                    if os.is_empty() {
                        String::from("none")
                    } else {
                        os.clone()
                    }
                ));

                x.to_string()
            }

            Build::Auto => {
                let (arch, os) = sys::get(build);

                Runa::starts(format!(
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

        let build_target = target_dir.join(&arch_os);

        if fs::create_dir_all(&build_target).is_err() {
            CompilerOutputs::fatal_internal(
                "Can't create architecture target folder."
            );
        }

        let nodes = visitor::work(&contexts, &mut symbols);

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

        let extra = {
            if let Some(m) = &mcu { m.clone() }
            else { "".into() }
        };

        owners.push(owner.clone());

        println!("{} {}", "#".yellow().bold(), owner);
        let _ = io::stdout().flush();

        let tx = tx.clone();

        threads.push(thread::spawn(move || {
            let start = Instant::now();

            let output = ProcessCommand::new(worker)
                .arg("--morgana-worker")
                .arg(&morgana_temp)
                .arg(&build_target)
                .arg(&owner)
                .arg(&sake)
                .arg(&extra)
                .output();

            let _ = fs::remove_file(&morgana_temp);

            let elapsed = start.elapsed().as_secs_f64();

            let result = match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(
                        &output.stdout
                    ).into_owned();

                    let stderr = String::from_utf8_lossy(
                        &output.stderr
                    ).into_owned();

                    let status = output.status.code();
                    let success = status == Some(0) && stderr.is_empty();

                    BuildResult {
                        owner,
                        success,
                        status,
                        stdout,
                        stderr,
                        elapsed,
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
                    elapsed,
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
                "{} {} {}",
                "✔️".b_green(),
                result.owner,
                format!("{:.3}s", result.elapsed).blue()
            );
        } else {
            print!(
                "{} {} {}",
                "x".b_red(),
                result.owner,
                format!("{:.3}s", result.elapsed).blue()
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
                elapsed: 0.0,
            });
        }
    }

    let total_elapsed = total_start.elapsed().as_secs_f64();

    println!(
        "Total compilation time: {}",
        format!("{:.3}s", total_elapsed).b_green()
    );

    Runa::deinit();

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
});

pub const BUILD: Command = command! {
    name: "build",
    description: "Compile your project",
    since: FIRST_RELEASE,
    example: "carla build -t xtensa_lx7 -M esp32",
    flags: {
        Evaluated("constexpr", "C") => "Lua Defines file",
        Evaluated("main", "m") => "Entrypoint file",

        Evaluated("extra-mcu", "M") => "Board name",
        Evaluated("target", "t") => "Output architecture/os",
    },
    function: build,
};
