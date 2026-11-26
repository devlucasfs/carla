use std::{
    process::Command,
    path::PathBuf,
    env
};

use crate::{
    utils::compiler_outputs::CompilerOutputs,
    package_manager::Provider::{
        Bitbucket,
        Codeberg,
        Github
    }
};

pub mod install;
pub mod update;
const BRANCHES: [&str; 2] = ["main", "master"];

#[derive(Debug, Clone, PartialEq)]
pub enum Provider {
    Github,
    Codeberg,
    Bitbucket,
}

fn curl(url: &str, extra: &[&str]) -> Result<std::process::Output, String> {
    Command::new("curl")
        .args(["-fsSL", "--connect-timeout", "10"])
        .args(extra)
        .args(["--", url])
        .output()
        .map_err(|e| format!("Failed to run curl: {e}"))
}

fn fetch(url: &str) -> Result<Option<String>, String> {
    let out = curl(url, &[])?;

    match out.status.code() {
        Some(0) => String::from_utf8(out.stdout)
            .map(Some)
            .map_err(|e| format!("Invalid UTF-8 from {url}: {e}")),
        Some(22) => Ok(None),
        _ => Err(format!(
            "curl failed for {url}: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        )),
    }
}

fn exists(url: &str) -> Result<bool, String> {
    let out = curl(url, &["-I"])?;

    match out.status.code() {
        Some(0) => Ok(true),
        Some(22) => Ok(false),
        _ => Err(format!(
            "curl failed for {url}: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        )),
    }
}

impl Provider {
    pub fn from(p: &str) -> Option<Provider> {
        match p.to_lowercase().as_str() {
            "bitbucket" => Some(Bitbucket),
            "codeberg"  => Some(Codeberg),
            "github"    => Some(Github),
            _ => None,
        }
    }

    fn parse(pkg: &str) -> (&str, &str) {
        let Some((author, lib)) = pkg.split_once('@') else {
            CompilerOutputs::fatal(&format!("Invalid pkg ({pkg}) format."));
            unreachable!()
        };
        (author, lib)
    }

    fn raw_url(&self, author: &str, lib: &str, branch: &str, path: &str) -> String {
        match self {
            Self::Bitbucket => format!("https://bitbucket.org/{author}/{lib}/raw/{branch}/{path}"),
            Self::Codeberg  => format!("https://codeberg.org/{author}/{lib}/raw/branch/{branch}/{path}"),
            Self::Github    => format!("https://raw.githubusercontent.com/{author}/{lib}/{branch}/{path}"),
        }
    }

    pub fn contents(&self, pkg: &str, path: &str) -> Option<String> {
        let (author, lib) = Self::parse(pkg);
        for branch in BRANCHES {
            let url = self.raw_url(author, lib, branch, path);
            match fetch(&url) {
                Ok(Some(body)) => return Some(body),
                Ok(None) => continue,
                Err(e) => CompilerOutputs::log(&e),
            }
        }

        None
    }

    pub fn check_exists(&self, pkg: &str, path: &str) -> bool {
        let (author, lib) = Self::parse(pkg);
        for branch in BRANCHES {
            let url = self.raw_url(author, lib, branch, path);
            match exists(&url) {
                Ok(true) => return true,
                Ok(false) => continue,
                Err(e) => CompilerOutputs::log(&e),
            }
        }

        false
    }

    fn clone_url(&self, author: &str, lib: &str) -> String {
        match self {
            Self::Bitbucket => format!("https://bitbucket.org/{author}/{lib}.git"),
            Self::Codeberg  => format!("https://codeberg.org/{author}/{lib}.git"),
            Self::Github    => format!("https://github.com/{author}/{lib}.git"),
        }
    }

    fn packages_dir(global: bool) -> PathBuf {
        if !global {
            return env::current_dir()
                    .unwrap()
                    .join("local-packages");
        }

        env::var_os("HOME")
            .or_else(|| env::var_os("USERPROFILE"))
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".carla")
            .join("libs-src")
    }

    pub fn clone_repository(&self, pkg: &str, global: bool) -> Result<PathBuf, String> {
        let (author, lib) = Self::parse(pkg);
        let dest = Self::packages_dir(global);
        let target = dest.join(lib);

        if target.exists() {
            return Err(format!("{} already exists", target.display()));
        }

        std::fs::create_dir_all(&dest)
            .map_err(|e| format!("Failed to create {}: {e}", dest.display()))?;

        let url = self.clone_url(author, lib);

        let out = Command::new("git")
            .args(["clone", "--depth", "1", "--quiet", "--", &url])
            .arg(&target)
            .env("GIT_TERMINAL_PROMPT", "0")
            .output()
            .map_err(|e| format!("Failed to run git: {e}"))?;

        if out.status.success() {
            Ok(target)
        } else {
            Err(format!(
                "git clone failed for {url}: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ))
        }
    }
}
