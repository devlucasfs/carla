use crate::compiler::{
    iterator::ContextIterator,
    nesting::Context::Common,
    node::Node::{self, Expression},
    patterns::{
        binding::{self, Binding},
        body,
        expressions::{self, Compiled},
        types::CarlaType,
    },
    scanner::{
        Keywords::{Else, If},
        Token,
        TokenKind::{Keyword, Semi},
    },
    symbols::Symbols,
};

/// `if [let|var a, b := value;] <cond> <body> [else <body | if ...>]`
///
/// `init` is the optional Go-style binding. `let` names are visible in `cond`, both bodies and
/// every `else if` chain (the scope closes with the whole `if`); `var` names stay in the parent scope.
///
/// `then` / `otherwise` are plain node lists: a block, or one inline node.
/// `else if` is `otherwise = [Node::Conditional(..)]`.
/// `ty` is `Some` when the `if` is a value: it has an `else`, and both branches end
/// in an expression without `;` of the same type. Otherwise any branch value is discarded.
#[derive(Debug, Clone, PartialEq)]
pub struct Conditional {
    pub init: Option<Binding>,
    pub cond: Compiled,
    pub then: Vec<Node>,
    pub otherwise: Option<Vec<Node>>,
    pub ty: Option<CarlaType>,
}

fn value_ty(body: &[Node]) -> Option<CarlaType> {
    match body.last()? {
        Expression(c, true) => Some(c.expr().ty()),
        Node::Conditional(c) => c.ty.clone(),
        _ => None,
    }
}

fn peek_is(it: &mut ContextIterator<'_>, kind: crate::compiler::scanner::TokenKind) -> bool {
    matches!(it.peek(), Some(Common(tk)) if tk.kind == kind)
}

pub fn work(
    _: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    conditional(syms, iterator).map(Node::Conditional)
}

fn conditional(syms: &mut Symbols, it: &mut ContextIterator<'_>) -> Result<Conditional, String> {
    // `let` gets a scope that covers the whole `if`/`else` chain; it is closed even on error
    let opened = binding::peek(it) == Some(true);
    if opened {
        syms.push_scope();
    }
    let out = header_and_branches(syms, it);
    if opened {
        syms.pop_scope();
    }
    out
}

fn header_and_branches(
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Conditional, String> {
    let init = if binding::peek(it).is_some() {
        Some(binding::work(syms, it)?)
    } else {
        None
    };

    let cond = expressions::parse(syms, it)?;
    expressions::require_bool(&cond, syms, "the condition of an `if`")?;

    let then = body::work(syms, it, true)?;

    let mut otherwise: Option<(Vec<Node>, bool)> = None;
    if peek_is(it, Keyword(Else)) {
        it.next();
        otherwise = Some(else_branch(syms, it)?);
    }

    // `if a { } else { };` — the `;` after a block is optional
    let ends_braced = match &otherwise {
        Some((_, braced)) => *braced,
        None => then.braced,
    };
    if ends_braced && peek_is(it, Semi) {
        it.next();
    }

    let ty = match &otherwise {
        Some((other, _)) => match (value_ty(&then.nodes), value_ty(other)) {
            (Some(a), Some(b)) if a.to_string() == b.to_string() => Some(a),
            (Some(a), Some(b)) => {
                return Err(format!(
                    "the branches of the `if` have different types: `{}` and `{}` (use `as`)",
                    a.to_string(),
                    b.to_string()
                ));
            }
            _ => None,
        },
        None => None,
    };

    Ok(Conditional {
        init,
        cond,
        then: then.nodes,
        otherwise: otherwise.map(|(nodes, _)| nodes),
        ty,
    })
}

/// Returns the nodes and whether the branch ended with a `{ }` block.
fn else_branch(
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<(Vec<Node>, bool), String> {
    if peek_is(it, Keyword(If)) {
        it.next();
        // the nested `if` already consumed its own terminator
        let nested = conditional(syms, it)?;
        return Ok((vec![Node::Conditional(nested)], false));
    }

    let b = body::work(syms, it, false)?;
    Ok((b.nodes, b.braced))
}
