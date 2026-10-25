use crate::compiler::{
    iterator::ContextIterator,
    nesting::Context::{Block, Common},
    node::Node::{self, InternalType},
    patterns::{
        self, binding, body,
        expressions::{self, Compiled},
        incdec::{self, IncDec},
        types::{self, CarlaType},
    },
    scanner::{
        BinOP::{IterConcat, IterConcatEq},
        Keywords::Let,
        PBB::Brace,
        Token,
        TokenKind::{self, BinaryOperator, Colon, Identifier, Keyword, Semi},
    },
    symbols::{
        Symbol::{BType, TypeAlias},
        Symbols,
    },
    visitor,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ForLoop {
    C {
        init: Vec<Node>,
        cond: Option<Compiled>,
        step: Option<IncDec>,
        body: Vec<Node>,
    },
    Range {
        var: Option<String>,
        ty: CarlaType,
        start: Compiled,
        end: Compiled,
        inclusive: bool,
        body: Vec<Node>,
    },
}

struct RangeVar {
    ty: Option<CarlaType>,
    name: String,
}

fn peek_is(it: &mut ContextIterator<'_>, kind: TokenKind) -> bool {
    matches!(it.peek(), Some(Common(tk)) if tk.kind == kind)
}

fn is_type_name(tk: &Token, syms: &Symbols) -> bool {
    matches!(
        &tk.kind,
        Identifier(id) if matches!(syms.lookup(id), Some(BType(_) | TypeAlias(_)))
    )
}

pub fn work(
    _: &Token,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    syms.push_scope();
    let out = parse(syms, it);
    syms.pop_scope();
    out.map(Node::For)
}

fn parse(syms: &mut Symbols, it: &mut ContextIterator<'_>) -> Result<ForLoop, String> {
    if let Some(var) = range_variable(syms, it)? {
        return range(syms, it, Some(var));
    }

    let c_style = match it.peek() {
        Some(Common(tk)) => tk.kind == Semi || tk.kind == Keyword(Let) || is_type_name(tk, syms),
        _ => false,
    };

    if c_style {
        c_style_loop(syms, it)
    } else {
        range(syms, it, None)
    }
}

fn range_variable(
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Option<RangeVar>, String> {
    let start = it.position();

    let ty = match it.peek() {
        Some(Common(tk)) if tk.kind == Keyword(Let) => {
            it.next();
            None
        }
        Some(Common(tk)) if is_type_name(tk, syms) => {
            let Some(Common(first)) = it.next() else {
                return Ok(None);
            };
            match types::work(first, syms, it) {
                Ok(InternalType(ty)) => Some(ty),
                _ => {
                    it.def(start);
                    return Ok(None);
                }
            }
        }
        _ => return Ok(None),
    };

    if let Some(Common(name)) = it.next() {
        if let Identifier(id) = &name.kind {
            if peek_is(it, Colon) {
                it.next();
                return Ok(Some(RangeVar {
                    ty,
                    name: id.clone(),
                }));
            }
        }
    }

    it.def(start);
    Ok(None)
}

fn range(
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
    var: Option<RangeVar>,
) -> Result<ForLoop, String> {
    let first = expressions::parse(syms, it)?;

    let inclusive = match it.next() {
        Some(Common(tk)) if tk.kind == BinaryOperator(IterConcat) => false,
        Some(Common(tk)) if tk.kind == BinaryOperator(IterConcatEq) => true,
        _ => {
            return Err(
                "Expected `..` or `..=` in the range of a `for` (`for 0..10`, `for int i : 0..=9`)"
                    .into(),
            );
        }
    };

    let last = expressions::parse(syms, it)?;

    let hint = var.as_ref().and_then(|v| v.ty.as_ref());
    let (start, end, ty) = expressions::range_bounds(first, last, hint)?;

    let var = match var {
        Some(v) => Some(binding::declare(syms, &v.name, &ty)?),
        None => None,
    };

    let body = body::looped(syms, it)?;

    Ok(ForLoop::Range {
        var,
        ty,
        start,
        end,
        inclusive,
        body,
    })
}

fn c_style_loop(syms: &mut Symbols, it: &mut ContextIterator<'_>) -> Result<ForLoop, String> {
    let init = init(syms, it)?;
    let cond = condition(syms, it)?;
    let step = step(syms, it)?;
    let body = body::looped(syms, it)?;

    Ok(ForLoop::C {
        init,
        cond,
        step,
        body,
    })
}

fn init(syms: &mut Symbols, it: &mut ContextIterator<'_>) -> Result<Vec<Node>, String> {
    if peek_is(it, Semi) {
        it.next();
        return Ok(vec![]);
    }

    let err = "the first part of a C-style `for` must be a declaration (`int i = 0;`) or empty";
    let Some(Common(first)) = it.next() else {
        return Err(err.into());
    };

    let decl = patterns::handler(it, syms, first).unwrap();
    let Node::Declaration(var) = &decl else {
        return Err(err.into());
    };

    if var.hopeless {
        return Ok(vec![decl]);
    }

    let value = expressions::parse(syms, it)?;
    match it.next() {
        Some(Common(tk)) if tk.kind == Semi => {}
        _ => return Err("Expected `;` after the initial value of the `for`".into()),
    }

    let value = Node::Expression(value, false);
    visitor::infer_declaration(syms, Some(&decl), &value);
    Ok(vec![decl, value])
}

fn condition(syms: &mut Symbols, it: &mut ContextIterator<'_>) -> Result<Option<Compiled>, String> {
    if peek_is(it, Semi) {
        it.next();
        return Ok(None);
    }

    let cond = expressions::parse(syms, it)?;
    expressions::require_bool(&cond, syms, "the condition of a `for`")?;

    match it.next() {
        Some(Common(tk)) if tk.kind == Semi => Ok(Some(cond)),
        _ => Err("Expected `;` after the condition of the `for`".into()),
    }
}

fn step(syms: &mut Symbols, it: &mut ContextIterator<'_>) -> Result<Option<IncDec>, String> {
    match it.peek() {
        Some(Common(tk)) if tk.kind == Colon => return Ok(None),
        Some(Block(Brace, _)) => return Ok(None),
        _ => {}
    }

    let Some(Common(first)) = it.next() else {
        return Err("Expected a step (`i++`) or the body of the `for`".into());
    };

    incdec::parse(first, syms, it).map(Some)
}
