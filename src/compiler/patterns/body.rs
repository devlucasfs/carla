use crate::{
    compiler::{
        iterator::ContextIterator,
        nesting::Context::{Block, Common},
        node::Node::{self, Declaration, Expression},
        patterns::{self, expressions},
        scanner::{
            Keywords::Else,
            PBB::{Brace, Paren},
            TokenKind::{Colon, Comma, Identifier, Keyword, Semi},
        },
        symbols::{
            Symbol::{BType, TypeAlias},
            Symbols,
        },
        visitor,
    },
    utils::pbb,
};

/// The body of an `if`/`else` (and, in the next stages, `while`/`for`).
///
///   `{ ... }`            a block: it opens its own symbol scope
///   `: <statement>;`     one inline statement, no scope
///   `: <expression>`     one inline expression, no scope (see `allow_else`)
///
/// A body yields a value when its last node is `Expression(_, true)`
/// (an expression with no `;` after it).
pub struct Body {
    pub nodes: Vec<Node>,
    pub braced: bool,
}

/// `allow_else`: an inline expression may be ended by a following `else` instead of `;`
/// (`if a : 1 else 2;`). The `else` is left in the iterator for the caller.
pub fn work(
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
    allow_else: bool,
) -> Result<Body, String> {
    match it.next() {
        Some(Block(Brace, ctx)) => Ok(Body {
            nodes: visitor::block(ctx, syms),
            braced: true,
        }),
        Some(Common(tk)) if tk.kind == Colon => inline(syms, it, allow_else),
        Some(Block(used, _)) => Err(format!(
            "Expected `:` or {{}} but was found {}",
            pbb::become_block(used)
        )),
        Some(Common(tk)) => Err(format!("Expected `:` or {{}} but was found {:?}", tk.kind)),
        None => Err("Expected `:` or {} but was found EOF".into()),
    }
}

fn inline(
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
    allow_else: bool,
) -> Result<Body, String> {
    let err = "Expected a statement or an expression after `:`";

    // A name that is a type starts a declaration (`int x = ...`), not an expression.
    let is_expr = match it.peek() {
        None => return Err(err.into()),
        Some(Block(Paren, _)) => true,
        Some(Block(..)) => false,
        Some(Common(tk)) => {
            expressions::starts_with(&tk.kind)
                && !matches!(
                    &tk.kind,
                    Identifier(id) if matches!(syms.lookup(id), Some(BType(_) | TypeAlias(_)))
                )
        }
    };

    if !is_expr {
        let Some(Common(first)) = it.next() else {
            return Err(err.into());
        };

        let node = patterns::handler(it, syms, first).unwrap();
        if matches!(node, Declaration(_)) {
            return Err("a declaration can't be an inline `:` body; use `{ }`".into());
        }

        return Ok(Body {
            nodes: vec![node],
            braced: false,
        });
    }

    let compiled = expressions::parse(syms, it)?;
    let node = match it.peek() {
        Some(Common(tk)) if allow_else && tk.kind == Keyword(Else) => Expression(compiled, true),
        Some(Common(tk)) if matches!(tk.kind, Semi | Comma) => {
            it.next();
            Expression(compiled, false)
        }
        None => Expression(compiled, true),
        _ => return Err("Expected `;` after an inline body".into()),
    };

    Ok(Body {
        nodes: vec![node],
        braced: false,
    })
}

/// The body of a loop: like `work` (an inline expression needs `;`), `break` / `continue`
/// are valid inside it, and the `;` after a `{ }` body is optional.
pub fn looped(syms: &mut Symbols, it: &mut ContextIterator<'_>) -> Result<Vec<Node>, String> {
    syms.enter_loop();
    let body = work(syms, it, false);
    syms.leave_loop();
    let body = body?;

    if body.braced && matches!(it.peek(), Some(Common(tk)) if tk.kind == Semi) {
        it.next();
    }

    Ok(body.nodes)
}
