use crate::terminal::Command;

pub mod build;
pub mod version;
pub mod upload;
pub mod help;
pub mod init;

pub const COMMANDS: &[Command] = &[
    version::VERSION, help::HELP,

    init::INIT,

    build::BUILD,
    upload::UPLOAD,
];
