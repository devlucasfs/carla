use crate::compiler::{
    iterator::ContextIterator,
    nesting::Context::Common,
    node::Node::{self, Declaration, Declare, Function},
    patterns::{
        declaration,
        r#extern::Extern::Variable,
        lambda::{self, Arg, LambdaRunner},
        types::CarlaType,
    },
    scanner::Token,
    symbols::{Symbol::UnfoldFunction, Symbols},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Unfold {
    Func(Box<CarlaType>, Vec<Arg>, String),
}

pub fn work(
    _: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    let Some(Common(first)) = iterator.next() else {
        return Err(
            "Expected a declaration after unfold, but was found a context block or EOF".into(),
        );
    };

    let declaration = declaration::work(first, syms, iterator);
    let Ok(Declaration(decl)) = &declaration else {
        let Err(err) = declaration else {
            unreachable!()
        };

        return Err(format!(
            "Expected a declaration after extern declaration, but some error happened {}",
            err
        ));
    };

    syms.erase(&decl.identifier);

    let Some(decl_t) = &decl._type else {
        return Err("An unfold declaration need a explicit type".into());
    };

    if decl.hopeless {
        return Ok(Declare(Variable(decl_t.clone(), decl.identifier.clone())));
    }

    let Some(Common(token)) = iterator.next() else {
        return Err("After a = in a unfold you need to inform a function notation".into());
    };

    let function = lambda::work(token, syms, iterator);
    let Ok(Function(func)) = &function else {
        let Err(err) = function else { unreachable!() };

        return Err(format!(
            "Expected a function after = in unfold declaration, but some error happened {}",
            err
        ));
    };

    syms.push(
        decl.identifier.clone(),
        UnfoldFunction(decl_t.clone(), func.args.clone()),
    );

    if func.run != LambdaRunner::Foward {
        return Err("An unfold function can't hold a code context".into());
    }

    Ok(Node::Unfoldable(Unfold::Func(
        decl_t.clone(),
        func.args.clone(),
        decl.identifier.clone(),
    )))
}
