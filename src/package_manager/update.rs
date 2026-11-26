use std::{
    path::{Path, PathBuf},
    process::Command,
    env,
    fs,
};

use colorize::AnsiColor;

use crate::{
    package_manager::install,
    utils::compiler_outputs::CompilerOutputs,
};

/// Returns true when the local clone is already at the same commit as the remote.
fn is_up_to_date(path: &Path) -> bool {
    if !path.join(".git").exists() {
        return false;
    }

    // Quietly fetch the latest from origin
    let fetch_ok = Command::new("git")
        .arg("-C")
        .arg(path)
        .args(["fetch", "--quiet", "origin"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !fetch_ok {
        return false;
    }

    let head = Command::new("git")
        .arg("-C")
        .arg(path)
        .args(["rev-parse", "HEAD"])
        .output();

    // origin/HEAD is the most reliable after a normal clone
    let remote = Command::new("git")
        .arg("-C")
        .arg(path)
        .args(["rev-parse", "origin/HEAD"])
        .output();

    match (head, remote) {
        (Ok(head), Ok(remote))
            if head.status.success() && remote.status.success() =>
        {
            let head_hash = String::from_utf8_lossy(&head.stdout).trim().to_string();
            let remote_hash = String::from_utf8_lossy(&remote.stdout).trim().to_string();
            !head_hash.is_empty() && head_hash == remote_hash
        }
        _ => false,
    }
}

/// Mirrors exactly the path logic used by `Provider::packages_dir` + `join(lib)`.
fn resolve_repo_path(package_name: &str, global: bool) -> PathBuf {
    let dest = if !global {
        env::current_dir()
            .unwrap_or_else(|e| {
                CompilerOutputs::fatal(format!("Can't read current directory: {e}"));
                unreachable!()
            })
            .join("local-packages")
    } else {
        env::var_os("HOME")
            .or_else(|| env::var_os("USERPROFILE"))
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".carla")
            .join("libs-src")
    };

    dest.join(package_name)
}

pub fn service(
    package: String,
    provider_literal: String,
    global: bool,
) {
    // package format is author@lib
    let (_, package_name) = package.split_once('@').unwrap();

    let repo_path = resolve_repo_path(package_name, global);

    if repo_path.exists() {
        if is_up_to_date(&repo_path) {
            println!(
                "\n{}",
                "Everything is already up to date".green()
            );
            return;
        }

        // Different commit (or not a git repo) → wipe and reinstall
        fs::remove_dir_all(&repo_path).unwrap_or_else(|e| {
            CompilerOutputs::fatal(format!(
                "Failed to remove existing package directory: {e}"
            ));
            unreachable!()
        });
    }

    // Re-use the real install implementation
    install::service(package, provider_literal, global);
}
