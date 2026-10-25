use crate::compiler::{
    iterator::ContextIterator,
    nesting::Context::Common,
    node::Node,
    patterns::{expressions, types::CarlaType},
    scanner::{
        Token,
        TokenKind::{self, Dec, Identifier, Inc, Semi},
    },
    symbols::{Symbol::Variable, Symbols},
};

#[derive(Debug, Clone, PartialEq)]
pub struct IncDec {
    pub name: String,
    pub ty: CarlaType,
    pub increment: bool,
}

pub fn follows(it: &mut ContextIterator<'_>) -> bool {
    matches!(it.peek(), Some(Common(tk)) if matches!(tk.kind, TokenKind::Inc | TokenKind::Dec))
}

pub fn parse(
    token: &Token,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<IncDec, String> {
    let Identifier(id) = &token.kind else {
        return Err(format!("Expected a variable before ++/--, found {:?}", token.kind));
    };

    let Some(sym) = syms.lookup(id) else {
        return Err(format!("unknown symbol: '{id}'"));
    };

    let ty = match sym {
        Variable(Some(t), _, false) => (**t).clone(),
        Variable(None, _, false) => return Err(format!("variable '{id}' doesn't have a type yet")),
        _ => return Err(format!("'{id}' isn't a variable, it can't be incremented")),
    };

    if !expressions::is_integer(&ty) {
        return Err(format!(
            "can't apply ++/-- to `{id}`: `{}` isn't an integer type",
            ty.to_string()
        ));
    }

    let increment = match it.next() {
        Some(Common(tk)) if tk.kind == Inc => true,
        Some(Common(tk)) if tk.kind == Dec => false,
        _ => return Err("Expected ++ or -- after the variable".into()),
    };

    Ok(IncDec {
        name: id.clone(),
        ty,
        increment,
    })
}

pub fn work(
    token: &Token,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    let step = parse(token, syms, it)?;

    match it.next() {
        None => {}
        Some(Common(tk)) if tk.kind == Semi => {}
        _ => return Err("Expected `;` after ++/--".into()),
    }

    Ok(Node::IncDec(step))
}
