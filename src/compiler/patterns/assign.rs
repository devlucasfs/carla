use crate::{
    compiler::{
        iterator::ContextIterator,
        nesting::Context::Common,
        node::Node,
        patterns::{
            expressions::{self, BinOp, Compiled, Expr},
            incdec,
            reg::{RegRef, Slice},
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
    /// `reg[ax, 64] = ...` / `reg[0x25] |= ...` (a `reg_t`): the whole register.
    Reg(RegRef),
    /// `reg[ax, 64, 0..3] = 0b01;` (a `reg_s_t`): only those bits, `value` is a binary literal.
    RegSlice(RegRef, Slice),
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
/// reg[ax, 64] &= 0x0123;       reg[0x25] |= 0x01;       reg[ax, 64, 0..3] = 0b01;
/// ```
///
/// A compound assignment is already expanded by the parser: `x += 1` is stored as
/// `x = x + 1` inside `value`, so the code generator only evaluates `value` and stores it.
/// `ty` is the type of the place (the type of the value that gets stored).
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

const EXPECTED_SLICE: &str = "A `reg[.., lo..hi]` slice is set with `=` and a binary literal, e.g. `reg[ax, 64, 0..3] = 0b01;` (compound operators, `++` and `--` aren't allowed on a slice)";

/// `reg[..., lo..hi] = 0b...;`: only `=` with a binary literal that fits in the slice.
fn assign_slice(
    reg: &RegRef,
    slice: Slice,
    ty: &CarlaType,
    it: &mut ContextIterator<'_>,
) -> Result<Node, String> {
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
        target: Target::RegSlice(reg.clone(), slice),
        ty: ty.clone(),
        value: Compiled::from_expr(Expr::Integer(value, ty.clone())),
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

    if let Expr::RegSlice(reg, sl, ty) = &place {
        return assign_slice(reg, *sl, ty, it);
    }

    let (target, ty) = match &place {
        Expr::Reg(reg, ty) => (Target::Reg(reg.clone()), ty.clone()),
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

    let value = match it.next() {
        Some(Common(tk)) => match &tk.kind {
            Sign => expressions::assignment(&place, None, syms, it)?,
            CompoundAssign(op) => expressions::assignment(&place, Some(bin_op(op)), syms, it)?,
            Inc => expressions::step(&place, true, syms)?,
            Dec => expressions::step(&place, false, syms)?,
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
