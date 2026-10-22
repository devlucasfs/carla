use crate::{cbinds, compiler::{
    iterator::ContextIterator,
    nesting::Context::{Block, Common},
    node::Node,
    scanner::{PBB, Token, TokenKind::RunaString},
    symbols::Symbols,
}};


#[allow(unused_variables)]
fn run(_: &mut Symbols, iterator: &mut ContextIterator<'_>) -> Result<bool, String> {
    match iterator.next() {
        Some(Common(Token {
            kind: RunaString(runa_expr),
            line,
        })) => Ok(cbinds::Runa::constexpr(runa_expr)),
        _ => Err("Expected a Runa Code String into the constexpr keyword context block".into())
    }
}

pub fn work(
    _: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    let Some(Block(PBB::Paren, ctx)) = iterator.next() else {
        return Err(
            "After a constexpr you need to use a parenthesis context block with your constexpr"
                .into(),
        );
    };

    let mut it = ContextIterator::new(ctx.as_slice(), 0);
    match run(syms, &mut it) {
        Err(err) => return Err(err),
        Ok(false) => iterator.consume(),
        _ => {}
    }

    return Ok(Node::Nop);
}
