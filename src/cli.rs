use crate::utils::eva::{Eva, EvaValue};
use std::env;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BRData {
    pub main: String,
    pub format: String,
    pub constexpr: String,
    pub builds: Vec<Build>
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Command {
    Version,
    Upload(String, String, String),
    Build(BRData),
    Run(BRData),

    #[default]
    Help,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FatalCLIMessage {
    None,
    Message(String),
}

impl Default for FatalCLIMessage {
    fn default() -> Self {
        Self::Message(
            "You need enter with a action. If you don't know the acceptable actions, use: help."
                .into(),
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct CLI {
    pub message: FatalCLIMessage,
    pub command: Command,
    pub use_ram: bool,
    pub enable_then: bool
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BuildPattern {
    pub arch: String,
    pub os: String
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

#[cfg(any(target_os = "linux", target_os = "android"))]
const UPLOAD_DEVICE: &'static str = "/dev/ttyUSB0";

#[cfg(target_os = "windows")]
const UPLOAD_DEVICE: &'static str = "COM3";

#[cfg(target_os = "macos")]
const UPLOAD_DEVICE: &'static str = "/dev/tty.usbserial";

#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
const UPLOAD_DEVICE: &'static str = "/dev/cuaU0";

#[cfg(target_os = "openbsd")]
const UPLOAD_DEVICE: &'static str = "/dev/cua00";

#[cfg(target_os = "netbsd")]
const UPLOAD_DEVICE: &'static str = "/dev/dtyU0";

#[cfg(target_os = "illumos")]
const UPLOAD_DEVICE: &'static str = "/dev/term/a";

#[cfg(target_os = "haiku")]
const UPLOAD_DEVICE: &'static str = "/dev/ports/usb";

impl CLI {
    fn spawn_error(message: &str) -> Self {
        Self {
            message: FatalCLIMessage::Message(message.into()),
            ..Default::default()
        }
    }

    pub fn build() -> Self {
        let default = Self {
            ..Default::default()
        };

        let mut args: Vec<_> = env::args().skip(1).collect();
        if args.len() == 0 {
            return default;
        }

        let command = args.remove(0);

        let Ok(folder) = env::current_dir() else {
            return CLI::spawn_error("Current directory can't be read.");
        };

        let eva = folder.join("target.eva");

        match command.as_str() {
            "version" => Self {
                message: FatalCLIMessage::None,
                command: Command::Version,
                ..Default::default()
            },

            "upload" => {
                if args.len() == 0 {
                    return CLI::spawn_error("Enter a valid extensor to be uploaded");
                }

                let mut device   = String::from(UPLOAD_DEVICE);
                let mut baudrate = String::from("115200");
                let     target   = args.remove(0);

                while args.len() > 0 {
                    let arg = args.remove(0);
                    let flags = ["--device", "--baudrate"];

                    match arg.as_str() {
                        "--device" => device = args.get(0).unwrap_or(&UPLOAD_DEVICE.into()).clone(),
                        "--baudrate" => baudrate = args.get(0).unwrap_or(&UPLOAD_DEVICE.into()).clone(),
                        _ => {}
                    }

                    if flags.contains(&arg.as_str()) {
                        args.remove(0);
                    }
                }

                Self {
                    message: FatalCLIMessage::None,
                    command: Command::Upload(target, device, baudrate),
                    ..Default::default()
                }
            },

            "build" | "run" => {
                let path = eva.display().to_string();
                let eva = Eva::open(&path);

                let EvaValue::String(main) = eva.get("target", "main") else {
                    return CLI::spawn_error(
                        "The `main` field in the target.eva file need to be a string.",
                    );
                };

                let EvaValue::String(constexpr) = eva.get("target", "constexpr") else {
                    return CLI::spawn_error(
                        "The `constexpr` field in the target.eva file need to be a string.",
                    );
                };

                let EvaValue::Map(output) = eva.get("target", "output") else {
                    return CLI::spawn_error(
                        "The `output` field in the target.eva file need to be a map.",
                    );
                };

                let EvaValue::String(format) = output.get("format") else {
                    return CLI::spawn_error(
                        "The `format` field in the output map need to be a string.",
                    );
                };

                let EvaValue::List(builds) = eva.get("target", "builds") else {
                    return CLI::spawn_error(
                        "The `builds` field in the target.eva file need to be a list.",
                    );
                };

                let mut patterns = vec![];
                for i in 0..builds.len() {
                    match builds.get(i) {
                        EvaValue::String(s) if s.to_lowercase() == "auto" => patterns.push(Build::Auto),
                        EvaValue::String(s) => patterns.push(Build::Pattern(BuildPattern { arch: s.clone(), os: String::new() })),
                        EvaValue::Map(map) => {
                            let EvaValue::String(os) = map.get("os") else {
                                return CLI::spawn_error(
                                    "The `os` field in the build pattern map need to be a string.",
                                );
                            };

                            let EvaValue::String(arch) = map.get("arch") else {
                                return CLI::spawn_error(
                                    "The `arch` field in the build pattern map need to be a string.",
                                );
                            };

                            patterns.push(Build::Pattern(BuildPattern { arch, os }));
                        }
                        _ => return CLI::spawn_error(
                            &format!("the {}th argument its invalid.", i+1)
                        )
                    }
                }

                let data = BRData {
                    builds: patterns,
                    constexpr: constexpr,
                    format: format,
                    main: main,
                };

                Self {
                    enable_then: args.contains(&"--samar-then".into()),
                    use_ram: args.contains(&"--no-disk".into()),
                    message: FatalCLIMessage::None,
                    command: if command == "build" {
                        Command::Build(data)
                    } else {
                        Command::Run(data)
                    },
                }
            }

            _ => default,
        }
    }
}
