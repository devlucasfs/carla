use crate::compiler::{
    iterator::ContextIterator,
    nesting::Context::Common,
    node::Node::{self, Expression, STMTDefer},
    patterns::expressions,
    scanner::Token,
    symbols::Symbols,
};

pub fn work(
    _: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    let err = "Enter a valid expression after defer".into();
    let Some(Common(next)) = iterator.next() else {
        return Err(err);
    };

    if !expressions::starts_with(&next.kind) {
        return Err(err);
    }

    let Ok(Expression(expr, eof)) = expressions::work(next, syms, iterator) else {
        return Err(err);
    };

    if eof {
        return Err("A defer is a comptime keyword, it value can't be returned".into());
    }

    Ok(STMTDefer(expr))
}
