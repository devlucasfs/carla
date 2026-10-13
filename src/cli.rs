use crate::utils::eva::{Eva, EvaValue};
use std::env;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BRData {
    pub main: String,
    pub format: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Command {
    Version,
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
}

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

        let args: Vec<_> = env::args().skip(1).collect();
        let Some(command) = args.get(0) else {
            return default;
        };

        let Ok(folder) = env::current_dir() else {
            return CLI::spawn_error("Current directory can't be read.");
        };

        let eva = folder.join("target.eva");

        match command.as_str() {
            "version" => Self {
                message: FatalCLIMessage::None,
                command: Command::Version,
            },

            "build" | "run" => {
                let path = eva.display().to_string();
                let eva = Eva::open(&path);

                let EvaValue::String(main) = eva.get("target", "main") else {
                    return CLI::spawn_error(
                        "The `main` field in the target.eva file need to be a string.",
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

                let data = BRData {
                    format: format,
                    main: main,
                };

                Self {
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
