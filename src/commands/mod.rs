use crate::terminal::Command;

pub mod build;
pub mod version;
pub mod upload;
pub mod help;

pub const COMMANDS: &[Command] = &[
    version::VERSION, help::HELP,

    build::BUILD,
    upload::UPLOAD
];
