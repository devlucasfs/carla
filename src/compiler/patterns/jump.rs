use crate::compiler::{
    iterator::ContextIterator,
    nesting::Context::Common,
    node::Node,
    scanner::{
        Keywords::{Break, Continue},
        Token,
        TokenKind::{Keyword, Semi},
    },
    symbols::Symbols,
};

pub fn work(
    token: &Token,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    let (node, word) = match &token.kind {
        Keyword(Break) => (Node::Break, "break"),
        Keyword(Continue) => (Node::Continue, "continue"),
        other => return Err(format!("Expected break or continue, found {other:?}")),
    };

    if !syms.in_loop() {
        return Err(format!("`{word}` outside of a loop"));
    }

    match it.next() {
        None => {}
        Some(Common(tk)) if tk.kind == Semi => {}
        _ => return Err(format!("Expected `;` after `{word}`")),
    }

    Ok(node)
}
