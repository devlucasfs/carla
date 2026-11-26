use std::{env, process::Command};

use colorize::AnsiColor;

use crate::{
    utils::compiler_outputs::CompilerOutputs,
    package_manager::Provider
};

pub fn service(package: String, provider_literal: String, global: bool) {
    let provider = Provider::from(&provider_literal).unwrap_or_else(|| {
        CompilerOutputs::log("Provider was not found. Using the default (Github)");
        Provider::Github
    });

    if !provider.check_exists(&package, "target.eva") {
        CompilerOutputs::fatal("Invalid package. That repository is not a carla project.");
        unreachable!()
    };

    let path = provider.clone_repository(&package, global).unwrap_or_else(|e| {
        CompilerOutputs::fatal(e);
        unreachable!()
    });

    _ = env::set_current_dir(&path);

    let status = Command::new("carla")
        .arg("build")
        .arg(if global { "--global-pkg" } else { "--local-pkg" })
        .current_dir(&path)
        .status()
        .unwrap_or_else(|e| {
            CompilerOutputs::fatal(format!("Failed to run carla build: {e}"));
            unreachable!()
        });

    if !status.success() {
        CompilerOutputs::fatal(format!("carla build failed in {package}"));
        unreachable!()
    }

    println!("\n{}", format!("Installed {package}").green());
}
