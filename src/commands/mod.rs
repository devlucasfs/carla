use std::process;

use crate::cli::{CLI, Command};

pub mod build;
pub mod version;
pub mod upload;

impl Command {
    pub fn run_command(&self, params: &CLI) {
        match params.command {
            Command::Version => self.version(params),
            Command::Upload(_, _, _, _) => self.upload(params),
            Command::Build(_, _) => self.build(params),
            _ => {}
        }
        process::exit(0)
    }
}
