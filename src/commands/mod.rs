use crate::terminal::Command;

pub mod build;
pub mod version;
pub mod upload;
pub mod help;
pub mod init;
pub mod pkg;

pub const COMMANDS: &[Command] = &[
    version::VERSION, help::HELP,

    init::INIT, pkg::PKG,

    build::BUILD,
    upload::UPLOAD,
];
