use std::{
    path::{Path, PathBuf},
    process::Command,
    env,
    fs
};

use colorize::AnsiColor;

use crate::{
    utils::compiler_outputs::CompilerOutputs,
    package_manager::Provider,
};

const ARTIFACT_EXTENSIONS: &[&str] = &[
    "so",
    "a",
    "dll",
    "lib",
    "mylib",
];

fn copy_artifacts(
    source: &Path,
    destination: &Path,
    package: &str,
) -> std::io::Result<usize> {
    let mut copied = 0;

    if !source.exists() { return Ok(0); }

    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            copied += copy_artifacts(
                &path,
                destination,
                package,
            )?;

            continue;
        }

        let Some(extension) = path.extension().and_then(|x| x.to_str())
        else { continue; };

        if !ARTIFACT_EXTENSIONS.contains(&extension) { continue; }

        let extension = extension.to_lowercase();

        let filename = if extension == "a" || extension == "so" {
            format!(
                "{}{}.{}",
                if package.to_lowercase().starts_with("lib") {
                    ""
                } else {
                    "lib"
                },
                package,
                extension
            )
        } else { format!("{package}.{extension}") };

        fs::copy(
            &path,
            destination.join(filename),
        )?;

        copied += 1;
    }

    Ok(copied)
}

pub fn service(
    package: String,
    provider_literal: String,
    global: bool,
) {
    let (_, package_name) = package.split_once('@').unwrap();

    let provider = Provider::from(&provider_literal).unwrap_or_else(|| {
        CompilerOutputs::log("Provider was not found. Using the default (Github)");
        Provider::Github
    });

    if !provider.check_exists(&package, "target.eva") {
        CompilerOutputs::fatal("Invalid package. That repository is not a carla project.");
        unreachable!()
    }

    let path = provider.clone_repository(&package, global).unwrap_or_else(|e| {
        CompilerOutputs::fatal(e);
        unreachable!()
    });

    let cwd = env::current_dir().unwrap();
    _ = env::set_current_dir(&path);

    let status =
        Command::new("carla")
            .arg("build")
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

    _ = env::set_current_dir(cwd);

    let destination = if global {
        env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                CompilerOutputs::fatal("Can't locate the user's home directory.");
                unreachable!()
            })
            .join(".carla")
            .join("shared")
    } else {
        env::current_dir()
            .unwrap_or_else(|e| {
                CompilerOutputs::fatal(format!("Can't read current directory: {e}"));
                unreachable!()
            })
            .join("libs")
    };

    let source = path.join("target");

    let copied = copy_artifacts(&source, &destination, &package_name).unwrap_or_else(|e| {
        CompilerOutputs::fatal(format!("Failed to copy package artifacts: {e}"));
        unreachable!()
    });

    if copied == 0 {
        CompilerOutputs::log("No shared or static library artifacts were found.");
    }

    println!(
        "\n{}",
        format!("Installed {package} ({copied} artifact(s))").green()
    );
}
