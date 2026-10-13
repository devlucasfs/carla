use std::{env, fs};

use crate::{
    cli::{CLI, Command},
    compiler::{charsert, codegen::Codegen, nesting, scanner::Scanner, symbols::Symbols, visitor},
    utils::compiler_outputs::CompilerOutputs,
};

impl Command {
    pub fn build(&self, _: &CLI) {
        let Ok(current_dir) = env::current_dir() else {
            CompilerOutputs::fatal("Can't read the current directory.");
            unreachable!();
        };

        let target = &current_dir.join("target");
        if !fs::exists(target).unwrap_or(false) {
            if let Err(_) = fs::create_dir_all(&target) {
                CompilerOutputs::fatal_internal("Can't create target folder.");
            }
        }

        let Command::Build(data) = self else {
            unreachable!()
        };

        let Ok(scan) = fs::read_to_string(&data.main) else {
            CompilerOutputs::fatal("Can't read the main file.");
            unreachable!()
        };

        let mut scanner = Scanner::new(scan);
        scanner.read();

        let mut symbols = Symbols::new();
        symbols.push_scope();
        charsert::append(&mut symbols);

        let contexts = nesting::work(scanner.tokens);
        let nodes = visitor::work(&contexts, &mut symbols);
        let code = Codegen::new(nodes.clone()).generate();
        println!("{nodes:#?}\n\n{code}");
    }
}
