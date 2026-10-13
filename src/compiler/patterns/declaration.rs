use crate::compiler::{
    iterator::ContextIterator,
    nesting::Context::Common,
    node::Node::{self, InternalType},
    patterns::types::{self, CarlaType},
    scanner::{
        Token,
        TokenKind::{Identifier, Semi, Sign},
    },
    symbols::{Symbol, Symbols},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub _type: Box<CarlaType>,
    pub identifier: String,
    pub hopeless: bool,
}

pub fn work(
    token: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    let Ok(InternalType(ty)) = types::work(token, syms, iterator) else {
        return Err("Every declaration starts with a type.".into());
    };

    let err: String = "A identifier is needed in a declaration".into();
    let Some(Common(identifier)) = iterator.next() else {
        return Err(err);
    };

    let Identifier(name) = &identifier.kind else {
        return Err(err);
    };

    match iterator.next() {
        Some(Common(tk)) if [Sign, Semi].contains(&tk.kind) => {
            syms.push(
                name,
                Symbol::Variable(Box::from(ty.clone()), tk.kind == Sign),
            );

            return Ok(Node::Declaration(Variable {
                identifier: name.clone(),
                _type: Box::from(ty.clone()),
                hopeless: false,
            }));
        }
        _ => todo!(),
    };
}
