use crate::compiler::{
    iterator::ContextIterator,
    nesting::Context::Common,
    node::Node::{self, Declaration, Declare, Function},
    patterns::{
        declaration,
        r#extern::Extern::{Func, Variable},
        lambda::{self, Arg, LambdaRunner},
        types::CarlaType,
    },
    scanner::Token,
    symbols::{Symbol::ExternFunction, Symbols},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Extern {
    Variable(Box<CarlaType>, String),
    Func(Box<CarlaType>, Vec<Arg>, String),
}

pub fn work(
    _: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    let Some(Common(first)) = iterator.next() else {
        return Err("Expected a token after extern, but was found a context block".into());
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

    if decl.hopeless {
        return Ok(Declare(Variable(
            decl._type.clone(),
            decl.identifier.clone(),
        )));
    }

    let Some(Common(token)) = iterator.next() else {
        return Err("After a = in a extern you need to inform a function notation".into());
    };

    let function = lambda::work(token, syms, iterator);
    let Ok(Function(func)) = &function else {
        let Err(err) = function else { unreachable!() };

        return Err(format!(
            "Expected a function after = in extern declaration, but some error happened {}",
            err
        ));
    };

    syms.push(
        decl.identifier.clone(),
        ExternFunction(decl._type.clone(), func.args.clone()),
    );

    if func.run != LambdaRunner::Foward {
        return Err("An extern function can't hold a code context".into());
    }

    Ok(Declare(Func(
        decl._type.clone(),
        func.args.clone(),
        decl.identifier.clone(),
    )))
}
