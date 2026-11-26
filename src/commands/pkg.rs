#[allow(unused_imports)]
use crate::{command, command_fn, terminal::Command, versions::FIRST_VERSION_EVER};
use crate::{
    utils::compiler_outputs::CompilerOutputs,
    package_manager
};

command_fn!(pkg(args) {
    install: String = "install",
    provider: String = "provider" || "github".into(),
    @bool global  = "global",
} => {
    match (install, provider, global) {
        (None, _, _) => CompilerOutputs::fatal("`pkg` alone ISN'T a valid command. Try to use `carla pkg --help` to listen all possible commands and their examples"),
        (Some(pkg), p, g) => package_manager::install::service(pkg, p, g)
    }
});

pub const PKG: Command = command! {
    name: "pkg",
    description: "Manage your packages into your Carla project",
    since: FIRST_VERSION_EVER,
    example: "",
    flags: {
        Evaluated("provider", "p") => "Package provider. Eg. Github",
        Evaluated("install", "i") => "Package you want to install",
        Boolean("global", "g") => "Global flag; Use it to install the package in the libs path (recommended)",
    },
    function: pkg,
};
