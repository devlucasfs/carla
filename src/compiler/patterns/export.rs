use crate::compiler::{
    iterator::ContextIterator,
    node::Node,
    scanner::Token,
    symbols::Symbols,
};

pub fn work(
    _: &Token,
    _: &mut Symbols,
    _: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    Ok(Node::Export)
}
