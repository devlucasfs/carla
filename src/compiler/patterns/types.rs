use crate::{
    compiler::{
        abi, charsert,
        iterator::ContextIterator,
        nesting::{self, Context},
        node::Node,
        scanner::{
            BitOp::OR,
            MathOp::Star,
            PBB::Bracket,
            Scanner, Token,
            TokenKind::{self, BitOperator, Comma, Dot, Identifier, MathOperator, Wild},
        },
        symbols::{
            Symbol::{self, BType, TypeAlias},
            Symbols,
        },
        visitor,
    },
    morgana::MorganaType::{self, Tuple},
    utils::compiler_outputs::CompilerOutputs,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Base {
    pub id: String,
    pub symbol: Symbol,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CarlaType {
    pub tuple: Vec<CarlaType>,
    pub levels: usize,
    pub base: Option<Base>,
    pub dim: Vec<i128>,
}

impl CarlaType {
    pub fn from<T: ToString>(data: T, syms: &mut Symbols) -> Self {
        let mut scan = Scanner::new(data.to_string());
        scan.read();
        let contexts = nesting::work(scan.tokens);
        let nodes = visitor::work(&contexts, syms);

        let err = format!(
            "{} isn't a valid carla type. You can't make a `from`",
            data.to_string()
        );

        if nodes.len() != 1 {
            CompilerOutputs::fatal_internal(&err);
        }

        let Node::InternalType(data) = nodes.get(0).unwrap() else {
            CompilerOutputs::fatal_internal(err);
            unreachable!();
        };

        data.clone()
    }

    pub fn from_basic<T: ToString>(data: T) -> Self {
        let mut syms = Symbols::new();
        charsert::append(&mut syms);

        let mut scan = Scanner::new(data.to_string());
        scan.read();
        let contexts = nesting::work(scan.tokens);
        let nodes = visitor::work(&contexts, &mut syms);

        let err = format!(
            "{} isn't a valid carla type. You can't make a `from`",
            data.to_string()
        );

        if nodes.len() != 1 {
            CompilerOutputs::fatal_internal(&err);
        }

        let Node::InternalType(data) = nodes.get(0).unwrap() else {
            CompilerOutputs::fatal_internal(err);
            unreachable!();
        };

        data.clone()
    }

    pub fn base_id(&self) -> Option<&str> {
        self.base.as_ref().map(|b| b.id.as_str())
    }

    /// The elements of this type when it is a tuple, looking through aliases
    /// (`using pair = |int, int|;` makes `pair` destructurable too).
    /// Pointers (`levels > 0`) and arrays (`dim`) of a tuple are not tuples.
    pub fn tuple_items(&self) -> Option<&[CarlaType]> {
        if self.levels > 0 || !self.dim.is_empty() {
            return None;
        }

        if !self.tuple.is_empty() {
            return Some(&self.tuple);
        }

        match &self.base {
            Some(Base {
                symbol: TypeAlias(inner),
                ..
            }) => inner.tuple_items(),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        if !self.tuple.is_empty() {
            result.push('|');

            for (i, ty) in self.tuple.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }

                result.push_str(&ty.to_string());
            }

            result.push('|');
        } else if let Some(base) = &self.base {
            result.push_str(&base.id);
        } else {
            result.push_str("| |");
        }

        for dim in &self.dim {
            result.push_str(&format!("[{dim}]"));
        }

        for _ in 0..self.levels {
            result.push('*');
        }

        result
    }

    pub fn named(id: &str, syms: &Symbols) -> Option<Self> {
        let symbol = syms.lookup(id)?;
        if !matches!(symbol, BType(_) | TypeAlias(_)) {
            return None;
        }

        Some(Self {
            tuple: vec![],
            levels: 0,
            base: Some(Base {
                id: id.to_string(),
                symbol: symbol.clone(),
            }),
            dim: vec![],
        })
    }

    pub fn untyped_ptr() -> Self {
        Self {
            levels: 1,
            ..Default::default()
        }
    }

    pub fn pointer_to(&self) -> Self {
        let mut t = self.clone();
        t.levels += 1;
        t
    }

    pub fn to_morgana(&self) -> MorganaType {
        if self.levels > 0 {
            return MorganaType::Ptr;
        }

        if let Some(base) = &self.base {
            match &base.symbol {
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

    parse_base(token, syms, iterator, true).map(Node::InternalType)
}

/// Reads `Abcd(.Child)*` without requiring every `.` to be a type:
/// it stops right before a `.member` that isn't a type symbol, leaving the
/// `.` in the iterator so the expressions parser can treat it as an extended
/// function reference.
pub fn parse_path(
    token: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
) -> Result<CarlaType, String> {
    parse_base(token, syms, iterator, false)
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

/// `strict = true`: a `.member` that isn't a type symbol is an error
/// (a `.` after a type is never valid anywhere else in a type context).
/// `strict = false`: stop before that `.` and return the type read so far.
fn parse_base(
    token: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
    strict: bool,
) -> Result<CarlaType, String> {
    let Identifier(id) = &token.kind else {
        return Err(format!("Expected type identifier, found {:?}", token.kind));
    };

    let Some(symbol) = syms.lookup(id) else {
        return Err(format!("Unknown type: {id}"));
    };

    if !matches!(symbol, BType(_) | TypeAlias(_)) {
        return Err(format!("Unknown type: {id}"));
    }

    let mut ty = CarlaType {
        base: Some(Base {
            id: id.clone(),
            symbol: symbol.clone(),
        }),
        ..Default::default()
    };

    // Abcd.Efgh.Ijkl: every segment must be a type symbol
    while let Some(Context::Common(dot)) = iterator.peek() {
        if dot.kind != Dot {
            break;
        }

        let backup = iterator.position();
        iterator.next();

        let err = "Expected an identifier after '.'";
        let Some(Context::Common(member)) = iterator.next() else {
            return Err(err.into());
        };
        let Identifier(member_id) = &member.kind else {
            return Err(err.into());
        };

        let full = abi::extension_symbol(&ty, member_id);
        match syms.lookup(&full) {
            Some(s) if matches!(s, BType(_) | TypeAlias(_)) => {
                ty = CarlaType {
                    base: Some(Base {
                        id: full,
                        symbol: s.clone(),
                    }),
                    ..Default::default()
                };
            }
            _ if strict => {
                return Err(format!(
                    "`{member_id}` is not a type inside `{}`",
                    ty.to_string()
                ));
            }
            _ => {
                // not a type: give the `.` back to the expressions parser
                iterator.def(backup);
                return Ok(ty);
            }
        }
    }

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
            TokenKind::Integer(i) => dimensions.push(i),
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

    ty.dim = dimensions;
    ty.levels = levels;
    Ok(ty)
}
