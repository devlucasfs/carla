#[allow(unused_imports)]
use crate::{command, command_fn, terminal::Command, versions::FIRST_VERSION_EVER};

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
        output: *mut c_char,
        libs: *mut c_char,
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

    if args.len() < 9 {
        eprintln!("Invalid Morgana worker arguments.");
        std::process::exit(2);
    }

    let path = CString::new(args[2].as_str()).unwrap();
    let target = CString::new(args[3].as_str()).unwrap();
    let extensor = CString::new(args[4].as_str()).unwrap();
    let sake = CString::new(args[5].as_str()).unwrap();
    let extra = CString::new(args[6].as_str()).unwrap();
    let output = CString::new(args[7].as_str()).unwrap();
    let libs = CString::new(args[8].as_str()).unwrap();

    unsafe {
        __morgana_build(
            path.as_ptr() as *mut c_char,
            target.as_ptr() as *mut c_char,
            extensor.as_ptr() as *mut c_char,
            sake.as_ptr() as *mut c_char,
            extra.as_ptr() as *mut c_char,
            output.as_ptr() as *mut c_char,
            libs.as_ptr() as *mut c_char,
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

fn parse_target(s: String) -> Build {
    if s.eq_ignore_ascii_case("auto") {
        Build::Auto
    } else {
        pattern_from_string(s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibKind {
    Shared,
    Static,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lib {
    pub kind: LibKind,
    pub name: String,
}

impl Lib {
    pub fn parse(s: &str) -> Result<Lib, String> {
        let Some((kind, name)) = s.split_once(':') else {
            return Err(format!("Invalid lib `{s}`: expected `<kind>:<name>`."));
        };

        let kind = match kind {
            "shared" => LibKind::Shared,
            "static" => LibKind::Static,
            other => return Err(format!("Unknown lib kind `{other}` in `{s}`.")),
        };

        if name.is_empty() {
            return Err(format!("Lib `{s}` has an empty name."));
        }

        Ok(Lib { kind, name: name.into() })
    }

    pub fn to_arg(&self) -> String {
        let kind = match self.kind {
            LibKind::Shared => "shared",
            LibKind::Static => "static",
        };
        format!("{kind}:{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactBuild {
    pub target: Build,
    pub outputs: Vec<String>,
    pub libs: Vec<Lib>,
}

fn default_outputs() -> Vec<String> {
    vec!["native-bin".into()]
}

const OUTPUT_FORMATS: [&str; 3] = ["native-bin", "static-archive", "shared-object"];

fn validate_output(output: &str) -> Result<(), String> {
    if OUTPUT_FORMATS.contains(&output) {
        Ok(())
    } else {
        Err(format!("Unknown output format `{output}`."))
    }
}

fn parse_outputs(value: EvaValue, n: usize) -> Result<Vec<String>, String> {
    let EvaValue::List(outputs) = value else {
        return Err(format!("The `outputs` field in artifact {n} must be a list."));
    };

    let mut formats = Vec::new();
    for j in 0..outputs.len() {
        let EvaValue::String(output) = outputs.get(j) else {
            return Err(format!("The {}th output in artifact {n} must be a string.", j + 1));
        };

        validate_output(&output)?;
        formats.push(output);
    }

    if formats.is_empty() {
        return Err(format!("Artifact {n} has no output formats."));
    }

    Ok(formats)
}

/// `native-bin,shared-object` -> ["native-bin", "shared-object"]
fn parse_outputs_arg(s: &str) -> Result<Vec<String>, String> {
    let formats: Vec<String> = s
        .split(',')
        .map(str::trim)
        .filter(|f| !f.is_empty())
        .map(String::from)
        .collect();

    if formats.is_empty() {
        return Err("The `--output-format` flag has no output formats.".into());
    }

    for format in &formats {
        validate_output(format)?;
    }

    Ok(formats)
}

fn parse_libs(value: EvaValue, n: usize) -> Result<Vec<Lib>, String> {
    let EvaValue::List(libs) = value else {
        return Err(format!("The `libs` field in artifact {n} must be a list."));
    };

    let mut out = Vec::new();
    for j in 0..libs.len() {
        let EvaValue::String(lib) = libs.get(j) else {
            return Err(format!("The {}th lib in artifact {n} must be a string.", j + 1));
        };

        out.push(Lib::parse(&lib).map_err(|e| format!("Artifact {n}: {e}"))?);
    }

    Ok(out)
}

/// `shared:foo,static:bar` -> [Lib, Lib]. Empty string = no libs.
fn parse_libs_arg(s: &str) -> Result<Vec<Lib>, String> {
    s.split(',')
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| Lib::parse(l).map_err(|e| format!("`--libs` flag: {e}")))
        .collect()
}

pub fn build_patterns(eva: &Eva) -> Result<Vec<ArtifactBuild>, String> {
    let EvaValue::List(artifacts) = eva.get("target", "artifacts") else {
        return Err("The `artifacts` field in target.eva must be a list.".into());
    };

    let mut patterns = Vec::new();

    for i in 0..artifacts.len() {
        let n = i + 1;

        let artifact = match artifacts.get(i) {
            EvaValue::String(target) => ArtifactBuild {
                target: parse_target(target),
                outputs: default_outputs(),
                libs: Vec::new(),
            },

            EvaValue::Map(map) => {
                let target = match map.get("target") {
                    EvaValue::String(t) => parse_target(t),
                    EvaValue::Nil => Build::Auto,
                    _ => return Err(format!("The `target` field in artifact {n} must be a string.")),
                };

                let outputs = match map.get("outputs") {
                    EvaValue::Nil => default_outputs(),
                    other => parse_outputs(other, n as usize)?,
                };

                let libs = match map.get("libs") {
                    EvaValue::Nil => Vec::new(),
                    other => parse_libs(other, n as usize)?,
                };

                ArtifactBuild { target, outputs, libs }
            }

            _ => return Err(format!("The {n}th artifact entry must be a string or a map.")),
        };

        patterns.push(artifact);
    }

    Ok(patterns)
}

command_fn!(build(args) {
    mcu:           String = "extra-mcu",
    target:        String = "target",
    constexpr:     String = "constexpr",
    entry:         String = "main",
    libs:          String = "libs",
    output_format: String = "output-format",
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

    let mut artifacts = match build_patterns(eva) {
        Ok(x) => x,
        Err(e) => {
            CompilerOutputs::fatal(e);
            unreachable!()
        }
    };

    if let Some(target) = target {
        for artifact in &mut artifacts {
            artifact.target = pattern_from_string(target.clone());
        }
    }

    if let Some(formats) = output_format {
        let outputs = match parse_outputs_arg(&formats) {
            Ok(x) => x,
            Err(e) => {
                CompilerOutputs::fatal(e);
                unreachable!()
            }
        };

        for artifact in &mut artifacts {
            artifact.outputs = outputs.clone();
        }
    }

    if let Some(libs) = libs {
        let libs = match parse_libs_arg(&libs) {
            Ok(x) => x,
            Err(e) => {
                CompilerOutputs::fatal(e);
                unreachable!()
            }
        };

        for artifact in &mut artifacts {
            artifact.libs = libs.clone();
        }
    }

    let jobs: Vec<(Build, String, Vec<Lib>)> = artifacts
        .into_iter()
        .flat_map(|artifact| {
            let ArtifactBuild { target, outputs, libs } = artifact;

            outputs.into_iter()
                .map(move |format| (target.clone(), format, libs.clone()))
        })
        .collect();

    println!(
        "{} Building {} artifact job(s)",
        "└─".b_grey(),
        jobs.len().to_string().b_yellow()
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
    let total = jobs.len();

    let output_width = jobs
        .iter()
        .map(|(_, output, _)| output.len())
        .max()
        .unwrap_or(0);

    let (tx, rx) = mpsc::channel::<(usize, BuildResult)>();

    let mut threads = Vec::new();
    let mut owners = Vec::new();

    for (index, (build, output_format, libs)) in jobs.iter().enumerate() {
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

        let target_name = arch_os.clone();

        let padded_output = format!(
            "{output_format:<output_width$}"
        );

        let owner = format!(
            "{arch_os}  {}",
            padded_output.b_cyan()
        );

        let build_target = target_dir
            .join(&arch_os)
            .join(output_format);

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

        let owner = owner.clone();
        let target_name = target_name.clone();
        let output_format = output_format.clone();
        let worker = executable.clone();
        let build_target = build_target.clone();
        let sake = sake.clone();

        let libs_arg = libs
            .iter()
            .map(Lib::to_arg)
            .collect::<Vec<_>>()
            .join(",");

        let extra = {
            if let Some(m) = &mcu {
                m.clone()
            } else {
                "".into()
            }
        };

        owners.push(owner.clone());

        println!(
            "{} {}  {}",
            "#".yellow().bold(),
            target_name.clone().b_grey().bold(),
            output_format.clone().b_cyan()
        );

        let _ = io::stdout().flush();

        let tx = tx.clone();

        threads.push(thread::spawn(move || {
            let start = Instant::now();

            let output = ProcessCommand::new(worker)
                .arg("--morgana-worker")
                .arg(&morgana_temp)
                .arg(&build_target)
                .arg(&target_name)
                .arg(&sake)
                .arg(&extra)
                .arg(&output_format)
                .arg(&libs_arg)
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
                "{} {}  {}",
                "✔️".b_green(),
                result.owner,
                format!("{:.3}s", result.elapsed).blue()
            );
        } else {
            print!(
                "{} {}  {}",
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
    since: FIRST_VERSION_EVER,
    example: "carla build -t xtensa_lx7 -M esp32 -o native_bin",
    flags: {
        Evaluated("constexpr", "C") => "Lua Defines file",
        Evaluated("main", "m") => "Entrypoint file",

        Evaluated("mcu", "M") => "Board name",
        Evaluated("target", "t") => "Output architecture/os",
        Evaluated("output-format", "o") => "Output formats, comma separated (overrides target.eva)",
        Evaluated("libs", "l") => "Libs as <kind>:<name>, comma separated (overrides target.eva)",
    },
    function: build,
};
