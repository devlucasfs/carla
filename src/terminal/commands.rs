#[allow(unused_imports)]

use crate::{
    {command, command_fn},
    terminal::Command
};

command_fn!(build(args) {
    output: String = "output",
    jobs: usize = "jobs" or 4,
    @bool verbose = "verbose",
} => {
    if verbose {
        println!("jobs = {jobs}");
    }
    if let Some(out) = output {
        println!("saída: {out}");
    }
});

pub const BUILD: Command = command! {
    name: "build",
    description: "Compila o projeto",
    since: "0.1.0",
    example: "tool build -o out --jobs 8 -v",
    flags: {
        Evaluated("output", "o") => "Arquivo de saída",
        Evaluated("jobs", "j")   => "Número de threads",
        Boolean("verbose", "v")  => "Modo verboso",
    },
    function: build,
};
