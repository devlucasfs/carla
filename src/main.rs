use crate::{cli::FatalCLIMessage, utils::compiler_outputs::CompilerOutputs};

pub mod cli;
pub mod commands;
pub mod compiler;
pub mod morgana;
pub mod utils;
pub mod cbinds;

fn main() {
    /* Collect and format all data */
    let params = cli::CLI::build();
    if let FatalCLIMessage::Message(msg) = &params.message {
        CompilerOutputs::fatal(msg);
    }

    params.command.run_command(&params);
}
