use crate::{
    compiler::{
        iterator::ContextIterator,
        nesting::Context::Common,
        node::Node,
        patterns::{
            expressions::{self, BinOp, Compiled, Expr},
            incdec,
            reg::{Loc, Slice},
            types::CarlaType,
        },
        scanner::{
            AssignOp, Keywords, Token,
            TokenKind::{
                self, BinaryLiteral, CompoundAssign, Dec, Hexa, Identifier, Inc, Integer, Keyword,
                Octal, Semi, Sign,
            },
        },
        symbols::Symbols,
    },
    morgana::MorganaType,
};

/// What an assignment writes to.
#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    /// `x = ...`
    Var(String),
    /// `p.* = ...` / `p[i] = ...`: the expression is the ADDRESS (`p`, or `p + i * size`).
    Deref(Compiled),
    /// `reg[reg, ax, 64] = ...` / `reg[io, 0x25] |= ...` / `reg[vreg, xmm0, 128] = ...`:
    /// the whole location.
    Reg(Loc),
    /// Part of a location:
    ///  * `reg[reg, ax, 64, 0..3] = 0b01;` only those bits, `value` is a binary literal;
    ///  * `reg[vreg, ymm0, 256, u32, 5] = x + 1;` a whole SIMD lane, which is a value of the
    ///    lane's type and accepts any expression, compound operators, `++` and `--`.
    ///
    /// The slice is always the absolute bit range.
    RegSlice(Loc, Slice),
}

/// Assignment to a place:
///
/// ```text
/// x = 312;      x += 123;     // x -= 12;  x *= 2;  x /= 2;  x %= 2;
/// x &= 1;  x |= 2;  x ^= 4;  x <<= 1;  x >>= 1;
/// p.* = 1;      p[i] = q[i];  p[i] += 1;  p[i]++;
/// ```
///
/// ```text
/// reg[reg, ax, 64] &= 0x0123;      reg[io, 0x25] |= 0x01;      reg[reg, ax, 64, 0..3] = 0b01;
/// reg[mem, 0x2000, 32, wo] = 1;    reg[vreg, ymm0, 256, u32, 5] += 1;
/// ```
///
/// A compound assignment is already expanded by the parser: `x += 1` is stored as
/// `x = x + 1` inside `value`, so the code generator only evaluates `value` and stores it.
/// `ty` is the type of the place (the type of the value that gets stored).
///
/// Access modes (`ro` / `wo` / `rw`) are checked here, at the statement, so the error points
/// at the assignment: `=` needs write access, everything that also reads the old value
/// (compound operators, `++`, `--`, a bit slice, a lane) needs both.
#[derive(Debug, Clone, PartialEq)]
pub struct Assign {
    pub target: Target,
    pub ty: CarlaType,
    pub value: Compiled,
}

/// Can a statement that starts with this token be an assignment? The caller that picks the
/// handler for a statement has to use this (identifiers, and `reg[...]`) before [`follows`].
pub fn starts(kind: &TokenKind) -> bool {
    matches!(kind, Identifier(_) | Keyword(Keywords::Reg))
}

/// True when the statement that starts with the identifier (or `reg`) we just read assigns something:
/// there is a `=`, `+=`, ..., `++` or `--` before the next `;`. Blocks (`[..]`, `(..)`, `{..}`)
/// are opaque, so only the top level of the statement is looked at. Nothing is consumed.
pub fn follows(it: &mut ContextIterator<'_>) -> bool {
    let start = it.position();
    let mut found = false;

    while let Some(ctx) = it.next() {
        if let Common(tk) = ctx {
            match tk.kind {
                Semi => break,
                Sign | CompoundAssign(_) | Inc | Dec => {
                    found = true;
                    break;
                }
                _ => {}
            }
        }
    }

    it.def(start);
    found
}

fn bin_op(op: &AssignOp) -> BinOp {
    match op {
        AssignOp::Add => BinOp::Add,
        AssignOp::Sub => BinOp::Sub,
        AssignOp::Mul => BinOp::Mul,
        AssignOp::Div => BinOp::Div,
        AssignOp::Rem => BinOp::Rem,
        AssignOp::And => BinOp::BitAnd,
        AssignOp::Or => BinOp::BitOr,
        AssignOp::Xor => BinOp::BitXor,
        AssignOp::Shl => BinOp::Shl,
        AssignOp::Shr => BinOp::Shr,
    }
}

const EXPECTED: &str = "Expected `=`, a compound operator (`+=`...), `++` or `--` after the left side of an assignment. The left side must be a variable, `p.*`, `p[i]` or `reg[...]` (`raw + i` is a value, not a place: use `raw[i] = ...`)";

const EXPECTED_SLICE: &str = "A `reg[KIND, .., lo..hi]` bit slice is set with `=` and a binary literal, e.g. `reg[reg, ax, 64, 0..3] = 0b01;` (compound operators, `++` and `--` aren't allowed on a bit slice; a whole SIMD lane is a value and accepts them)";

/// A slice that covers exactly one SIMD lane is a value of the lane's type, not a bit field.
// fn is_whole_lane(loc: &Loc, slice: &Slice) -> bool {
//     matches!(&loc.lane, Some(lane) if slice.width == lane.ty.bits)
// }

/// `reg[..., lo..hi] = 0b...;`: only `=` with a binary literal that fits in the slice.
fn assign_slice(
    loc: &Loc,
    slice: Slice,
    ty: &CarlaType,
    it: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    // setting some bits reads the old value, replaces them and writes it back
    loc.check_modify()?;

    match it.next() {
        Some(Common(tk)) if tk.kind == Sign => {}
        _ => return Err(EXPECTED_SLICE.into()),
    }

    let (value, digits) = match it.next() {
        Some(Common(tk)) => match &tk.kind {
            BinaryLiteral(v, digits) => (*v, *digits),
            Integer(_) | Hexa(_) | Octal(_) => {
                return Err(
                    "a slice can only be set with a binary literal (`0b...`), e.g. `= 0b01`".into(),
                )
            }
            _ => return Err(EXPECTED_SLICE.into()),
        },
        _ => return Err(EXPECTED_SLICE.into()),
    };

    if digits > slice.width {
        return Err(format!(
            "the binary literal has {digits} digits but the slice is only {} bit{} wide",
            slice.width,
            if slice.width == 1 { "" } else { "s" }
        ));
    }

    Ok(Node::Assign(Assign {
        target: Target::RegSlice(loc.clone(), slice),
        ty: ty.clone(),
        value: Compiled::from_expr(Expr::Integer(value, ty.clone())),
    }))
}

/// `reg[vreg, ymm0, 256, u32, 5] = x + 1;` / `+= 1` / `++`: a whole lane behaves like a
/// variable of the lane's type. The lane is written as a read-modify-write of the vector.
fn assign_lane(
    loc: &Loc,
    slice: Slice,
    ty: &CarlaType,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    loc.check_modify()?;

    // `RegSlice` can't be an operand, its readable twin (`.*`) can
    let read = Expr::RegSliceGet(loc.clone(), slice, ty.clone());

    let value = match it.next() {
        Some(Common(tk)) => match &tk.kind {
            Sign => expressions::assignment(&read, None, syms, it)?,
            CompoundAssign(op) => expressions::assignment(&read, Some(bin_op(op)), syms, it)?,
            Inc => expressions::step(&read, true, syms)?,
            Dec => expressions::step(&read, false, syms)?,
            _ => return Err(EXPECTED.into()),
        },
        _ => return Err(EXPECTED.into()),
    };

    Ok(Node::Assign(Assign {
        target: Target::RegSlice(loc.clone(), slice),
        ty: ty.clone(),
        value,
    }))
}

pub fn parse(
    token: &Token,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    // `x++` / `x--` on a plain variable keep their own node
    if matches!(token.kind, Identifier(_))
        && matches!(it.peek(), Some(Common(tk)) if matches!(tk.kind, Inc | Dec))
    {
        return incdec::parse(token, syms, it).map(Node::IncDec);
    }

    let place = expressions::parse_place(token, syms, it)?;

    match &place {
        Expr::RegSlice(loc, sl, ty) | Expr::RegSliceGet(loc, sl, ty) => {
            return if loc.is_whole_lane(sl) {
                assign_lane(loc, *sl, ty, syms, it)
            } else if matches!(place, Expr::RegSlice(..)) {
                assign_slice(loc, *sl, ty, it)
            } else {
                Err(EXPECTED.into())
            };
        }
        _ => {}
    }

    let (target, ty) = match &place {
        Expr::Reg(loc, ty) => (Target::Reg(loc.clone()), ty.clone()),
        Expr::Var(name, ty) => (Target::Var(name.clone()), ty.clone()),
        Expr::Deref(ptr, ty) => (
            Target::Deref(Compiled::from_expr((**ptr).clone())),
            ty.clone(),
        ),
        _ => return Err(EXPECTED.into()),
    };

    if matches!(ty.to_morgana(), MorganaType::Tuple(_)) {
        return Err(format!(
            "assigning isn't supported yet: `{}` is a struct/tuple (`store` is scalar only)",
            ty.to_string()
        ));
    }

    // only a whole location has an access mode to check here
    let loc = match &target {
        Target::Reg(l) => Some(l.clone()),
        _ => None,
    };

    let value = match it.next() {
        Some(Common(tk)) => match &tk.kind {
            Sign => {
                if let Some(l) = &loc {
                    l.check_store()?;
                }
                expressions::assignment(&place, None, syms, it)?
            }
            CompoundAssign(op) => {
                if let Some(l) = &loc {
                    l.check_modify()?;
                }
                expressions::assignment(&place, Some(bin_op(op)), syms, it)?
            }
            Inc => {
                if let Some(l) = &loc {
                    l.check_modify()?;
                }
                expressions::step(&place, true, syms)?
            }
            Dec => {
                if let Some(l) = &loc {
                    l.check_modify()?;
                }
                expressions::step(&place, false, syms)?
            }
            _ => return Err(EXPECTED.into()),
        },
        _ => return Err(EXPECTED.into()),
    };

    Ok(Node::Assign(Assign { target, ty, value }))
}

pub fn work(
    token: &Token,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    let node = parse(token, syms, it)?;

    match it.next() {
        None => {}
        Some(Common(tk)) if tk.kind == Semi => {}
        _ => return Err("Expected `;` after the assignment".into()),
    }

    Ok(node)
}
