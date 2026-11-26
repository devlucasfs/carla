use std::fmt;

use super::{Args, Command, Flag};
use crate::utils::eva::{Eva, EvaValue};

pub const POSITIONAL: &str = "_";
pub const TARGET_FILE: &str = "./target.eva";
pub const DEFAULT_NAMESPACE: &str = "target";

pub const PLATFORM_MARKER: &str = "platform:";

#[derive(Debug, PartialEq)]
pub enum ParseError {
    NoCommand,
    UnknownCommand(String),
    UnknownFlag(String),
    MissingValue(String),
    UnexpectedValue(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoCommand => write!(f, "No command found. Try to use \"help\" to see which commands are available."),
            Self::UnknownCommand(c) => write!(f, "Unknown command: `{c}`; Check all commands using help!"),
            Self::UnknownFlag(x) => write!(f, "Unknown flag: `{x}`"),
            Self::MissingValue(x) => write!(f, "The flag `{x}` needs a value."),
            Self::UnexpectedValue(x) => write!(f, "The flag `{x}` does not need a value."),
        }
    }
}

impl std::error::Error for ParseError {}

fn find_flag<'a>(cmd: &'a Command, name: &str, short: bool) -> Option<&'a Flag> {
    cmd.flags.iter().map(|(f, _)| f).find(|f| match f {
        Flag::Evaluated(l, s) | Flag::Boolean(l, s) => {
            if short { *s == name } else { *l == name }
        }
        Flag::None => false,
    })
}

fn apply<'a>(
    flag: &Flag,
    shown: &str,
    inline: Option<String>,
    it: &mut impl Iterator<Item = &'a String>,
    args: &mut Args,
) -> Result<(), ParseError> {
    match flag {
        Flag::Evaluated(long, _) => {
            let value = match inline {
                Some(v) => v,
                None => it
                    .next()
                    .cloned()
                    .ok_or_else(|| ParseError::MissingValue(shown.to_string()))?,
            };
            args.push((long.to_string(), value));
        }
        Flag::Boolean(long, _) => {
            if inline.is_some() {
                return Err(ParseError::UnexpectedValue(shown.to_string()));
            }
            args.push((long.to_string(), "true".to_string()));
        }
        Flag::None => unreachable!(),
    }
    Ok(())
}

fn platform_from_key(key: &str) -> Option<&str> {
    key.strip_prefix(PLATFORM_MARKER).filter(|ns| !ns.is_empty())
}

fn platform_namespace(argv: &[String]) -> Option<&str> {
    argv.iter()
        .skip(1)
        .take_while(|a| a.as_str() != "--")
        .filter_map(|a| a.strip_prefix("--"))
        .map(|rest| rest.split_once('=').map_or(rest, |(k, _)| k))
        .filter_map(platform_from_key)
        .last()
}

pub fn parse<'a>(
    commands: &'a [Command],
    argv: &[String],
) -> Result<(&'a Command, Args), ParseError> {
    let mut it = argv.iter();

    let name = it.next().ok_or(ParseError::NoCommand)?;
    let cmd = commands
        .iter()
        .find(|c| c.name == name)
        .ok_or_else(|| ParseError::UnknownCommand(name.clone()))?;

    let mut args = Args::new();
    let mut only_positional = false;

    while let Some(arg) = it.next() {
        if only_positional || arg == "-" || !arg.starts_with('-') {
            args.push((POSITIONAL.to_string(), arg.clone()));
            continue;
        }

        if arg == "--" {
            only_positional = true;
            continue;
        }

        if let Some(rest) = arg.strip_prefix("--") {
            let (key, inline) = match rest.split_once('=') {
                Some((k, v)) => (k, Some(v.to_string())),
                None => (rest, None),
            };

            if platform_from_key(key).is_some() {
                continue;
            }

            let flag = find_flag(cmd, key, false)
                .ok_or_else(|| ParseError::UnknownFlag(format!("--{key}")))?;
            apply(flag, &format!("--{key}"), inline, &mut it, &mut args)?;
        } else {
            let rest = &arg[1..];
            for (i, c) in rest.char_indices() {
                let key = c.to_string();
                let shown = format!("-{c}");
                let flag = find_flag(cmd, &key, true)
                    .ok_or_else(|| ParseError::UnknownFlag(shown.clone()))?;

                match flag {
                    Flag::Boolean(..) => apply(flag, &shown, None, &mut it, &mut args)?,
                    _ => {
                        let tail = &rest[i + c.len_utf8()..];
                        let tail = tail.strip_prefix('=').unwrap_or(tail);
                        let inline = (!tail.is_empty()).then(|| tail.to_string());
                        apply(flag, &shown, inline, &mut it, &mut args)?;
                        break;
                    }
                }
            }
        }
    }

    Ok((cmd, args))
}

/// `main` -> (default_ns, "main"); `x-y` -> ("x", "y").
fn default_location<'a>(long: &'a str, default_ns: &'a str) -> (&'a str, &'a str) {
    match long.split_once('-') {
        Some((ns, field)) if !ns.is_empty() && !field.is_empty() => (ns, field),
        _ => (default_ns, long),
    }
}

fn eva_to_string(v: EvaValue) -> Option<String> {
    match v {
        EvaValue::String(s) => Some(s),
        EvaValue::Number(n) => Some(n.to_string()),
        EvaValue::Boolean(b) => Some(b.to_string()),
        EvaValue::Map(_) | EvaValue::List(_) | EvaValue::Nil => None,
    }
}

pub fn apply_defaults(cmd: &Command, args: &mut Args, eva: &Eva, default_ns: &str) {
    for (flag, _) in cmd.flags {
        let (long, is_bool): (String, bool) = match flag {
            Flag::Evaluated(l, _) => (l.to_string(), false),
            Flag::Boolean(l, _) => (l.to_string(), true),
            Flag::None => continue,
        };

        if args.iter().any(|(k, _)| *k == long) {
            continue;
        }

        let (ns, field) = default_location(&long, default_ns);
        if !eva.has(ns, field) {
            continue;
        }

        match (is_bool, eva.get(ns, field)) {
            (true, EvaValue::Boolean(true)) => args.push((long, "true".to_string())),
            (true, _) => {}
            (false, v) => match eva_to_string(v) {
                Some(s) => args.push((long, s)),
                None => {}
            },
        }
    }
}

pub fn run(commands: &[Command], argv: &[String]) -> Result<(), ParseError> {
    let (cmd, mut args) = parse(commands, argv)?;

    let default_ns = platform_namespace(argv).unwrap_or(DEFAULT_NAMESPACE);

    match Eva::try_open(TARGET_FILE) {
        Ok(eva) => apply_defaults(cmd, &mut args, &eva, default_ns),
        Err(_) => {}
    }

    (cmd.function)(args);
    Ok(())
}

pub fn positionals(args: &Args) -> Vec<&str> {
    args.iter()
        .filter(|(k, _)| k == POSITIONAL)
        .map(|(_, v)| v.as_str())
        .collect()
}
