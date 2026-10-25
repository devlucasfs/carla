use crate::compiler::{
    iterator::ContextIterator,
    node::Node,
    patterns::{
        binding::{self, Binding},
        body,
        expressions::{self, Compiled},
    },
    scanner::Token,
    symbols::Symbols,
};

/// `while [let|var a, b := value;] <cond> <body>`
///
/// Each iteration runs: `init` (when present: the value is evaluated again and destructured),
/// then `cond`; the body runs while `cond` is true. `let` names live in the loop scope,
/// `var` names in the parent scope (declared once, assigned on every iteration).
#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoop {
    pub init: Option<Binding>,
    pub cond: Compiled,
    pub body: Vec<Node>,
}

pub fn work(
    _: &Token,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    let opened = binding::peek(it) == Some(true);
    if opened {
        syms.push_scope();
    }
    let out = header_and_body(syms, it);
    if opened {
        syms.pop_scope();
    }
    out.map(Node::While)
}

fn header_and_body(
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<WhileLoop, String> {
    let init = if binding::peek(it).is_some() {
        Some(binding::work(syms, it)?)
    } else {
        None
    };

    let cond = expressions::parse(syms, it)?;
    expressions::require_bool(&cond, syms, "the condition of a `while`")?;

    let body = body::looped(syms, it)?;
    Ok(WhileLoop { init, cond, body })
}
