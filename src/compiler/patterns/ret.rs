use crate::compiler::{
    iterator::ContextIterator,
    nesting::Context::Common,
    node::Node::{self, STMTReturn, STMTReturnTuple},
    patterns::expressions,
    scanner::{
        Token,
        TokenKind::{Comma, Semi},
    },
    symbols::Symbols,
};

pub fn work(
    _: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    match iterator.peek() {
        Some(Common(tk)) if expressions::starts_with(&tk.kind) => {}
        _ => return Err("Enter a valid expression after return".into()),
    }

    let mut values = vec![expressions::parse(syms, iterator)?];

    while matches!(iterator.peek(), Some(Common(tk)) if tk.kind == Comma) {
        iterator.next();
        values.push(expressions::parse(syms, iterator)?);
    }

    match iterator.next() {
        Some(Common(tk)) if tk.kind == Semi => {}
        _ => return Err("A return is a comptime keyword, it value can't be returned".into()),
    }

    if values.len() == 1 {
        Ok(STMTReturn(values.remove(0)))
    } else {
        Ok(STMTReturnTuple(values))
    }
}
