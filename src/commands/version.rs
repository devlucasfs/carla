#[allow(unused_imports)]
use crate::{command, command_fn, terminal::Command, versions::FIRST_VERSION_EVER};

pub const CARLA_VERSION: &'static str = "26.0";
const CARLA_PS: &'static str = "Rust rewrite";

command_fn!(version(args) {} => {
     println!("carla version {CARLA_VERSION} ({CARLA_PS})");
});

pub const VERSION: Command = command! {
    name: "version",
    description: "Show the installed Carla compiler version",
    since: FIRST_VERSION_EVER,
    example: "",
    flags: {},
    function: version,
};
