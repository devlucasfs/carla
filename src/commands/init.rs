use std::{env, fs};

use colorize::AnsiColor;

#[allow(unused_imports)]
use crate::{command, command_fn, terminal::Command, versions::FIRST_VERSION_EVER};
use crate::utils::compiler_outputs::CompilerOutputs;

command_fn!(init(args) {} => {
    let Ok(cwd) = env::current_dir() else {
        CompilerOutputs::fatal("Sorry, we can't read your current directory.");
        return;
    };

    let eva_file_path = cwd.join("target.eva");
    let eva_file_content = include_str!("../../default/target.eva");
    _ = fs::write(eva_file_path, eva_file_content);

    let sake_file_path = cwd.join("Sake");
    let sake_file_content = include_str!("../../default/Sake");
    _ = fs::write(sake_file_path, sake_file_content);

    let src_folder = cwd.join("src");
    _ = fs::create_dir(&src_folder);

    let main_file_path = src_folder.join("main.crl");
    let main_file_content = include_str!("../../default/main.crl");
    _ = fs::write(main_file_path, main_file_content);

    let core_folder = cwd.join("core");
    _ = fs::create_dir(&core_folder);

    let constexpr_file_path = core_folder.join("constexpr.lua");
    let constexpr_file_content = include_str!("../../default/constexpr.lua");
    _ = fs::write(constexpr_file_path, constexpr_file_content);

    println!("{}", "Your Carla project was created!".green().bold());
});

pub const INIT: Command = command! {
    name: "init",
    description: "Make the current folder a Carla project.",
    since: FIRST_VERSION_EVER,
    example: "",
    flags: {},
    function: init,
};
