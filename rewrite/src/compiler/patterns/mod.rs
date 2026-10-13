use colorize::AnsiColor;

use crate::{
    compiler::{
        iterator::ContextIterator,
        node::Node,
        scanner::{Keywords::Fn, Token, TokenKind::Keyword},
        symbols::Symbols,
    },
    utils::compiler_outputs::CompilerOutputs,
};

pub mod declaration;
pub mod lambda;
pub mod types;

macro_rules! try_pattern {
    ($iterator:expr, $errors:expr, $origin:expr, $result:expr) => {
        let backup = $iterator.position();
        match $result {
            Ok(value) => return Some(value),
            Err(err) => {
                $errors.push(($origin, err.to_string()));
                $iterator.def(backup);
            }
        }
    };
}

pub fn handler(
    iterator: &mut ContextIterator<'_>,
    syms: &mut Symbols,
    token: &Token,
) -> Option<Node> {
    let mut errors = vec![];

    if types::starts_with(&token.kind) {
        try_pattern!(
            iterator,
            errors,
            "Declaration parser",
            declaration::work(token, syms, iterator)
        );

        try_pattern!(
            iterator,
            errors,
            "Type parser",
            types::work(token, syms, iterator)
        );
    }

    match token.kind {
        Keyword(Fn) => {
            try_pattern!(
                iterator,
                errors,
                "Function parser",
                lambda::work(token, syms, iterator)
            );
        }
        _ => {}
    };

    let formated = errors
        .iter()
        .map(|f| {
            format!(
                "└─ {} - {}",
                f.1.clone().yellow(),
                format!("(By. {})", f.0.italic()).grey().bold()
            )
        })
        .collect::<Vec<String>>();

    CompilerOutputs::fatal(format!(
        "{}: Unknown pattern found in line {}:\n{} All received error messages:\n{}\n",
        "Syntax issue".cyan().bold(),
        token.line + 1,
        "~".black().bold(),
        formated.join("\n")
    ));

    None
}
