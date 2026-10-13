pub struct CompilerOutputs;

use std::process;

use colorize::*;

impl CompilerOutputs {
    pub fn fatal<T: ToString>(text: T) {
        println!("{}: {}", "[Carla FAIL]".red().bold(), text.to_string());
        process::exit(14);
    }

    pub fn fatal_internal<T: ToString>(text: T) {
        println!(
            "{}: {} {}",
            "[Carla FAIL]".red().bold(),
            "internal error!".yellow().bold(),
            text.to_string()
        );
        process::exit(14);
    }
}
