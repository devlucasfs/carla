use colorize::AnsiColor;

#[allow(unused_imports)]
use crate::{command, command_fn, terminal::{Command, Flag}, versions::FIRST_RELEASE};
use crate::{commands::{COMMANDS, version}, terminal::tricks::{TRICK_MSG, TRICKS}};

#[derive(Clone, Copy)]
enum WrapStyle {
    Plain,
    Indented,
}

fn print_wrapped(text: &str, width: usize, style: WrapStyle) {
    let emit = |line: &String| match style {
        WrapStyle::Plain => println!("{}", line.clone().grey().bold()),
        WrapStyle::Indented => println!("  {}", line.clone().b_grey().bold()),
    };

    let prefix_len = match style {
        WrapStyle::Plain => 0,
        WrapStyle::Indented => 2,
    };
    let max = width.saturating_sub(prefix_len);

    let mut line = String::new();

    for word in text.split_whitespace() {
        if !line.is_empty() && line.len() + word.len() + 1 > max {
            emit(&line);
            line.clear();
        }

        if !line.is_empty() {
            line.push(' ');
        }

        line.push_str(word);
    }

    if !line.is_empty() {
        emit(&line);
    }
}

fn flag_label(flag: &Flag) -> Option<String> {
    match flag {
        Flag::Evaluated(l, s) => Some(format!("-{s}, --{l} <value>")),
        Flag::Boolean(l, s) => Some(format!("-{s}, --{l}")),
        Flag::None => None,
    }
}

command_fn!(help(args) {} => {
    let width = COMMANDS
        .iter()
        .flat_map(|c| c.flags.iter())
        .filter_map(|(flag, _)| flag_label(flag))
        .map(|label| label.chars().count())
        .max()
        .unwrap_or(0);

    println!("Listing all availeable commands in Carla ({}):\n", version::CARLA_VERSION);
    for c in COMMANDS {
        if c.name == "help" { continue }
        let x = {
            if c.example.is_empty() { "".into() }
            else { format!("  {} eg. {}\n", "└─".grey().bold(), c.example.yellow().bold()) }
        };

        println!("  {:<12}: {}\n{}", c.name.b_red(), c.description.b_grey(), x);
        for (flag, desc) in c.flags {
            if let Some(label) = flag_label(flag) {
                println!("      {:<width$}  {desc}", label, width = width);
            }
        }
        println!();
    }

    println!("{}:", "tricks".b_magenta().bold());
    print_wrapped(TRICK_MSG, 57, WrapStyle::Plain);
    println!();

    for (name, description, example) in TRICKS {
        println!("  {:<12}:", name.green().bold());
        print_wrapped(description, 57, WrapStyle::Indented);

        println!("  {} eg. {}", "~>".grey().bold(), example.yellow().bold());
        println!();
    }
});

pub const HELP: Command = command! {
    name: "help",
    description: "List all available commands",
    since: FIRST_RELEASE,
    example: "",
    flags: {},
    function: help,
};
