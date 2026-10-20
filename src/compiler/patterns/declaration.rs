use crate::compiler::{
    iterator::ContextIterator,
    nesting::Context::Common,
    node::Node::{self, InternalType},
    patterns::types::{self, CarlaType},
    scanner::{
        Keywords::{Fn, Let},
        Token,
        TokenKind::{Identifier, Keyword, Semi, Sign, Walrus},
    },
    symbols::{Symbol, Symbols},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub _type: Option<Box<CarlaType>>,
    pub identifier: String,
    pub hopeless: bool,
}

pub fn work(
    token: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    let ty = if token.kind == Keyword(Let) {
        None
    } else {
        let Ok(InternalType(ty)) = types::work(token, syms, iterator) else {
            return Err("Every declaration starts with a type.".into());
        };

        Some(ty)
    };

    let err: String = "A identifier is needed in a declaration".into();
    let Some(Common(identifier)) = iterator.next() else {
        return Err(err);
    };

    let Identifier(name) = &identifier.kind else {
        return Err(err);
    };

    match iterator.next() {
        Some(Common(tk)) if [Sign, Semi, Walrus].contains(&tk.kind) => {
            if ty.is_none() && tk.kind != Walrus {
                return Err(
                    "In a `let` definition you you need to use := to define a value.".into(),
                );
            }

            let is_fn = tk.kind == Sign
                && matches!(iterator.peek(), Some(Common(next)) if next.kind == Keyword(Fn));

            let def_type = if ty.is_none() {
                None
            } else {
                Some(Box::from(ty.unwrap().clone()))
            };

            syms.push(
                name,
                Symbol::Variable(
                    def_type.clone(),
                    tk.kind == Sign || tk.kind == Walrus,
                    is_fn,
                ),
            );

            Ok(Node::Declaration(Variable {
                identifier: name.clone(),
                _type: def_type,
                hopeless: !(tk.kind == Sign || tk.kind == Walrus),
            }))
        }
        _ => todo!(),
    }
}
