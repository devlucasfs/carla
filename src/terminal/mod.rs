#[macro_use]
pub mod macros;
pub mod commands;
pub mod parser;
pub mod tricks;

pub type Args = Vec<(String, String)>;

#[derive(Debug, PartialEq, Default)]
pub enum Flag {
    Evaluated(&'static str, &'static str),
    Boolean(&'static str, &'static str),

    #[default]
    None
}

#[derive(Debug)]
pub struct Command {
    pub function: fn(Args),
    pub name: &'static str,
    pub flags: &'static [(Flag, &'static str)],
    pub description: &'static str,
    pub since: &'static str,
    pub example: &'static str
}

fn noop(_: Args) {}

impl Default for Command {
    fn default() -> Self {
        Command {
            function: noop,
            name: "",
            flags: &[],
            description: "",
            since: "",
            example: "",
        }
    }
}
