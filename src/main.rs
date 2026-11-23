use crate::{commands::COMMANDS, utils::compiler_outputs::CompilerOutputs};

pub mod commands;
pub mod terminal;
pub mod versions;
pub mod compiler;
pub mod morgana;
pub mod cbinds;
pub mod utils;

fn main() {
    if crate::commands::build::morgana_worker_entry() {
        return;
    }

    let args: Vec<_> = std::env::args().skip(1).collect();
    let slice = args.as_slice();

    if let Err(err) = terminal::parser::run(COMMANDS, slice) {
        CompilerOutputs::fatal(err.to_string());
    }
}
