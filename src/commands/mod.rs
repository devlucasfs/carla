use std::process;

use crate::cli::{CLI, Command};

pub mod build;
pub mod version;

impl Command {
    pub fn run_command(&self, params: &CLI) {
        match params.command {
            Command::Version => self.version(params),
            Command::Build(_) => self.build(params),
            _ => {}
        }
        process::exit(0)
    }
}
