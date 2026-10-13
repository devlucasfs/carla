use crate::{
    compiler::{
        iterator::ContextIterator,
        nesting::Context,
        node::Node,
        scanner::{
            BitOp::OR,
            MathOp::Star,
            PBB::Bracket,
            Token,
            TokenKind::{self, BitOperator, Comma, Identifier, MathOperator, Wild},
        },
        symbols::{
            Symbol::{self, BType, TypeAlias},
            Symbols,
        },
    },
    morgana::MorganaType::{self, Tuple},
};

#[derive(Debug, Clone, PartialEq)]
pub struct CarlaType {
    tuple: Vec<CarlaType>,
    levels: usize,
    base: Option<Symbol>,
    dim: Vec<u128>,
}

impl CarlaType {
    pub fn to_morgana(&self) -> MorganaType {
        if self.levels > 0 {
            return MorganaType::Ptr;
        }

        if let Some(sym) = &self.base {
            match sym {
                BType(btype) => return btype.clone(),
                TypeAlias(alias) => return alias.to_morgana(),
                _ => unreachable!(),
            }
        }

        if self.tuple.len() > 0 {
            let mut tuple = vec![];
            for ty in &self.tuple {
                tuple.push(ty.to_morgana());
            }
            return Tuple(tuple);
        }

        unreachable!()
    }
}

pub fn starts_with(kind: &TokenKind) -> bool {
    matches!(kind, BitOperator(OR) | Identifier(_))
}

pub fn work(
    token: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    if token.kind == BitOperator(OR) {
        return parse_tuple(syms, iterator);
    }

    parse_base(token, syms, iterator)
}

fn parse_tuple(syms: &mut Symbols, iterator: &mut ContextIterator<'_>) -> Result<Node, String> {
    let mut types = Vec::new();

    loop {
        let Some(Context::Common(token)) = iterator.next() else {
            return Err("Expected type in tuple".into());
        };

        if token.kind == Wild {
            let Some(Context::Common(close)) = iterator.next() else {
                return Err("Expected '|' after the wild operator in tuple type".into());
            };

            if close.kind != BitOperator(OR) {
                return Err("Expected '|' after the wild operator in tuple type".into());
            }

            return Ok(Node::InternalType(CarlaType {
                tuple: vec![],
                levels: 0,
                base: None,
                dim: vec![],
            }));
        }

        let Node::InternalType(ty) = work(token, syms, iterator)? else {
            unreachable!()
        };

        types.push(ty);

        let Some(next) = iterator.peek() else {
            return Err("Expected ',' or '|' after tuple type".into());
        };

        let Context::Common(token) = next else {
            return Err("Expected ',' or '|' after tuple type".into());
        };

        _ = match token.kind {
            Comma => iterator.next(),
            BitOperator(OR) => {
                iterator.next();
                break;
            }
            _ => {
                return Err(format!("Expected ',' or '|', found {:?}", token.kind));
            }
        };
    }

    Ok(Node::InternalType(CarlaType {
        tuple: types,
        levels: 0,
        base: None,
        dim: vec![],
    }))
}

fn parse_base(
    token: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    let Identifier(id) = &token.kind else {
        return Err(format!("Expected type identifier, found {:?}", token.kind));
    };

    let Some(symbol) = syms.lookup(id) else {
        return Err(format!("Unknown type: {id}"));
    };

    let mut dimensions = Vec::new();
    loop {
        let Some(op) = iterator.peek() else { break };
        let Context::Block(pbb, ctx) = op else { break };

        if *pbb != Bracket {
            break;
        }

        if ctx.len() != 1 {
            return Err("Array dimension requires exactly one Integer".into());
        }

        let Context::Common(size) = &ctx[0] else {
            return Err("Array dimension must contain an Integer".into());
        };

        match size.kind {
            TokenKind::Octal(i) | TokenKind::Hexa(i) => dimensions.push(i),
            TokenKind::Integer(i) => dimensions.push(i as u128),
            _ => return Err("Array dimension must contain an Integer".into()),
        }
        iterator.next();
    }

    let mut levels = 0;
    while let Some(Context::Common(tk)) = iterator.peek() {
        if tk.kind != MathOperator(Star) {
            break;
        }
        iterator.next();
        levels += 1;
    }

    Ok(Node::InternalType(CarlaType {
        tuple: Vec::new(),
        levels,
        base: Some(symbol.clone()),
        dim: dimensions,
    }))
}
