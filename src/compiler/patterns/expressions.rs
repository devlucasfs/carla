use crate::compiler::{
    iterator::ContextIterator,
    nesting::Context::{Block, Common},
    node::Node::{self, Expression},
    scanner::{
        BitOp::AND,
        MathOp::Minus,
        PBB::Paren,
        Token,
        TokenKind::{
            self, Bang, Comma, Dot, Float, Hexa, Identifier, Integer, MathOperator, Octal, Semi,
            String as Str,
        },
    },
    symbols::{
        Symbol::{ExternFunction, Variable},
        Symbols,
    },
};

macro_rules! unpack {
    ($data:expr) => {{
        match $data {
            Ok(a) => a,
            Err(e) => return Err(e),
        }
    }};
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Float(f64),
    Integer(i128),
    String(String),
    Pointer(String),
    Call(String, Vec<Node>),
}

pub fn starts_with(kind: &TokenKind) -> bool {
    matches!(
        kind,
        Identifier(_)
            | TokenKind::BitOperator(AND)
            | Integer(_)
            | Hexa(_)
            | Octal(_)
            | Float(_)
            | Str(_)
            | MathOperator(Minus)
            | Bang
            | Dot
    )
}

pub fn work(
    token: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    let data = match &token.kind {
        TokenKind::BitOperator(AND) => {
            let err = "Expected an identifier after a & (pointer operator)".into();
            let Some(Common(tk)) = iterator.next() else {
                return Err(err);
            };

            let Identifier(identifier) = &tk.kind else {
                return Err(err);
            };

            Expr::Pointer(identifier.clone())
        }

        Integer(i) | Hexa(i) | Octal(i) => Expr::Integer(i.clone()),

        Identifier(id) => {
            let err = format!("{id} isn't a valid identifier");
            let Some(data) = syms.lookup(id.clone()) else {
                return Err(format!("simbolo n existente : '{id}'"));
            };

            let fn_call = |_err: String,
                           iterator: &mut ContextIterator<'_>,
                           syms: &mut Symbols|
             -> Result<Expr, String> {
                let err = format!("Invalid {id} call");
                let Some(Block(pbb, ctx)) = iterator.next() else {
                    return Err(err);
                };

                if *pbb != Paren {
                    return Err(err);
                }

                let mut params = vec![];
                let mut internal = ContextIterator::new(ctx.as_slice(), 0);
                while internal.remaining() > 0 {
                    let Some(Common(first)) = internal.next() else {
                        return Err(err);
                    };

                    if !starts_with(&first.kind) {
                        return Err(err);
                    }

                    match work(&first, syms, &mut internal) {
                        Ok(data) => params.push(data),
                        Err(e) => return Err(e),
                    }
                }

                Ok(Expr::Call(id.clone(), params.clone()))
            };

            match data {
                Variable(ty, _, func) if *func => unpack!(fn_call(err, iterator, syms)),
                ExternFunction(_, _) => unpack!(fn_call(err, iterator, syms)),
                _ => return Err(err),
            }
        }

        _ => todo!(),
    };

    let Some(next) = iterator.next() else {
        return Ok(Expression(data, true));
    };

    let err = "Expected ; or , and also maybe EOF, after an expression".into();
    let Common(finish) = next else {
        return Err(err);
    };

    match finish.kind {
        Comma | Semi => Ok(Expression(data, false)),
        _ => Err(err),
    }
}
