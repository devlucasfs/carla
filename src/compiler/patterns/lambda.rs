use crate::{
    compiler::{
        iterator::ContextIterator,
        nesting::Context::{self, Block, Common},
        node::Node::{self, Expression, Function, InternalType, Nop},
        patterns::{
            expressions::{self, Compiled},
            lambda::LambdaRunner::{Expr, Foward, Scope},
            types::{self, CarlaType},
        },
        scanner::{
            Keywords::Mut,
            PBB::{Brace, Paren},
            Token,
            TokenKind::{Arrow, Comma, Identifier, Keyword, Semi},
        },
        symbols::Symbols,
    },
    utils::pbb,
};

#[derive(Debug, Clone, PartialEq)]
pub enum LambdaRunner {
    Scope(Vec<Context>),
    Expr(Compiled),
    Foward,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arg {
    pub ids: Vec<String>,
    pub ty: Box<CarlaType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lambda {
    pub public: bool,
    pub no_mangled: bool,
    pub extension: bool,
    pub mutable: bool,
    pub args: Vec<Arg>,
    pub run: LambdaRunner,
}

fn is_self(ctx: &Context) -> bool {
    matches!(ctx, Common(t) if matches!(&t.kind, Identifier(id) if id == "self"))
}

fn parse_self(iter: &mut ContextIterator<'_>, in_extend: bool) -> Result<(bool, bool), String> {
    let mut mutable = false;

    let has_self = match iter.peek() {
        Some(Common(tk)) if tk.kind == Keyword(Mut) => {
            iter.next();
            match iter.next() {
                Some(ctx) if is_self(ctx) => {}
                _ => return Err("Expected `self` after `mut`".into()),
            }
            mutable = true;
            true
        }
        Some(ctx) if is_self(ctx) => {
            iter.next();
            true
        }
        _ => false,
    };

    if !has_self {
        return Ok((false, false));
    }

    if !in_extend {
        return Err("`self` is only allowed in functions inside an extend block".into());
    }

    match iter.peek() {
        None => {}
        Some(Common(tk)) if tk.kind == Comma => {
            iter.next();
        }
        _ => return Err("Expected , or ) after self".into()),
    }

    Ok((true, mutable))
}

pub fn parse_args(
    iterator: &mut ContextIterator<'_>,
    syms: &mut Symbols,
) -> Result<Vec<Arg>, String> {
    let mut args: Vec<Arg> = vec![];
    while let Some(ctx) = iterator.next() {
        let Common(common) = ctx else {
            return Err("Context blocks are not allowed into argument blocks".into());
        };

        let Ok(ty) = types::work(common, syms, iterator) else {
            let Identifier(id) = &common.kind else {
                return Err(format!(
                    "Expected a type or a identifier into the argument block but was found {:?}",
                    common.kind
                ));
            };

            if args.len() == 0 {
                return Err("The first argument can't use an inferred type".into());
            }

            let last = args.last_mut().unwrap();
            last.ids.push(id.clone());

            if iterator.remaining() == 0 {
                break;
            }

            match iterator.next() {
                Some(Block(used, _)) => {
                    return Err(format!(
                        "Expected , or ) but was found {}",
                        pbb::become_block(used)
                    ));
                }
                Some(Common(tk)) if tk.kind != Comma => {
                    return Err(format!("Expected , or ) but was found {:?}", tk.kind));
                }
                None => return Err(format!("Expected , or ) but was found {:?}", Nop)),
                Some(_) => continue,
            }
        };

        let InternalType(internal) = ty else {
            unreachable!();
        };

        args.push(Arg {
            ids: vec![],
            ty: Box::from(internal),
        });
    }

    Ok(args)
}

fn typed_params(args: &[Arg]) -> Vec<(String, CarlaType)> {
    args.iter()
        .flat_map(|arg| {
            arg.ids
                .iter()
                .map(move |id| (id.clone(), arg.ty.as_ref().clone()))
        })
        .collect()
}

pub fn work(
    token: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    work_in(token, syms, iterator, false)
}

pub fn work_in(
    _: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
    in_extend: bool,
) -> Result<Node, String> {
    let err = "After a fn you need to use a parenthesis context block with arguments".into();
    let Some(Block(block, ctx)) = iterator.next() else {
        return Err(err);
    };

    if block != &Paren {
        return Err(err);
    }

    let mut iter = ContextIterator::new(ctx.as_slice(), 0);
    let (extension, mutable) = parse_self(&mut iter, in_extend)?;
    let args = parse_args(&mut iter, syms)?;

    match iterator.next() {
        Some(Common(semi)) if semi.kind == Semi => Ok(Function(Lambda {
            run: Foward,
            no_mangled: false,
            public: true,
            extension,
            mutable,
            args,
        })),
        Some(Common(arrow)) if arrow.kind == Arrow => {
            let err = "Enter a valid expression in the function return".into();
            let Some(Common(next)) = iterator.next() else {
                return Err(err);
            };

            if !expressions::starts_with(&next.kind) {
                return Err(err);
            }

            let mut params = typed_params(&args);
            if extension {
                params.insert(0, ("self".into(), syms.get_extended().unwrap().clone()));
            }

            let e = syms.temporary(&params, |s| expressions::work(next, s, iterator));

            let Ok(Expression(expr, eof)) = e else {
                return e;
            };

            if eof {
                return Err(
                    "A single line function return value can't be finished with EOF or EOC".into(),
                );
            }

            Ok(Function(Lambda {
                run: Expr(expr),
                no_mangled: false,
                public: true,
                extension,
                mutable,
                args,
            }))
        }
        Some(Block(ppb, body_ctx)) if ppb == &Brace => Ok(Function(Lambda {
            run: Scope(body_ctx.clone()),
            no_mangled: false,
            public: true,
            extension,
            mutable,
            args,
        })),
        Some(Block(used, _)) => Err(format!(
            "Expected -> or {{}} but was found {}",
            pbb::become_block(used)
        )),
        Some(Common(tk)) => Err(format!("Expected -> or {{}} but was found {:?}", tk.kind)),
        None => Err("Expected -> or {} but was found EOF".into()),
    }
}
