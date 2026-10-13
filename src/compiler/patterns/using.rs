use crate::{
    compiler::{
        iterator::ContextIterator,
        nesting::Context::{Block, Common},
        node::Node::{self, InternalType, Nop},
        patterns::types,
        scanner::{
            Token,
            TokenKind::{Identifier, Semi, Sign},
        },
        symbols::{Symbol, Symbols},
    },
    utils::pbb,
};

pub fn work(
    _: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    let err: String = "A identifier is needed in a type alias".into();
    let Some(Common(identifier)) = iterator.next() else {
        return Err(err);
    };

    let Identifier(name) = &identifier.kind else {
        return Err(err);
    };

    match iterator.next() {
        Some(Common(tk)) if tk.kind == Sign => {
            let Some(Common(token)) = iterator.next() else {
                return Err("A type was expected after the = in the type alias".into());
            };

            let Ok(InternalType(ty)) = types::work(token, syms, iterator) else {
                return Err("A type alias need a valid type after the =".into());
            };

            syms.push(name, Symbol::TypeAlias(Box::from(ty)));

            let err =
                "After define the type alias you need to use a ; to finish the expression".into();
            let Some(Common(semi)) = iterator.next() else {
                return Err(err);
            };

            if semi.kind != Semi {
                return Err(err);
            }

            return Ok(Nop);
        }
        Some(Common(tk)) => Err(format!("Expected a =, but was found {:?}", tk.kind)),
        Some(Block(d, _)) => Err(format!(
            "Expected a =, but was found {}",
            pbb::become_block(d)
        )),
        None => Err("Expected a =, but was found EOF".into()),
    }
}
