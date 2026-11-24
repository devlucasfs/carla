use std::{
    cell::RefCell,
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
};

use crate::{
    compiler::{
        abi,
        iterator::ContextIterator,
        nesting::Context::{self, Block, Common},
        node::Node::{self, Expression},
        patterns::{
            lambda::Arg,
            reg::{self, Loc, Slice},
            types::{self, Base, CarlaType, Variadic},
        },
        scanner::{
            BinOP as Cmp,
            BitOp::{AND, LSH, OR, RSH, XOR},
            Keywords::{As, Fn as FnKw, Reg, RegOk, Syscall},
            MathOp::{self, Minus},
            PBB::{Bracket, Paren},
            Token,
            TokenKind::{
                self, And, Bang, BinaryOperator, BitOperator, Bool, Comma, DoubleColon, Dot,
                Float, Hexa, Identifier, Integer, Keyword, MathOperator, Nil, Octal, Or, Semi,
                String as Str,
            },
        },
        symbols::{
            Symbol::{self, BType, ExternFunction, TypeAlias, UnfoldFunction, Variable},
            Symbols,
        },
    },
    morgana::MorganaType,
};

const DEFAULT_INT: &str = "int";
const DEFAULT_BOOL: &str = "bool";
const DEFAULT_CSTRZ: &str = "c_strz";
const DEFAULT_SYSCALL_RET: &str = "int";
const DEFAULT_FN_REF: &str = "fn_ref";
const DEFAULT_VOID: &str = "void";
const DEFAULT_COUNT: &str = "ulong";
const DEFAULT_STRING: &str = "string";
const DEFAULT_TO_STRING: &str = "to_string";

const BARE_SLICE: &str = "a bit slice `reg[KIND, target, .., lo..hi]` isn't a value: set it with `= 0b...` or read it with `.*` (a whole SIMD lane, without a range, is a value and needs neither)";
mod bp {
    pub const LOG_OR: (u8, u8) = (1, 2);
    pub const LOG_AND: (u8, u8) = (3, 4);
    pub const COMPARE: (u8, u8) = (5, 6);
    pub const BIT_OR: (u8, u8) = (7, 8);
    pub const BIT_XOR: (u8, u8) = (9, 10);
    pub const BIT_AND: (u8, u8) = (11, 12);
    pub const SHIFT: (u8, u8) = (13, 14);
    pub const ADDITIVE: (u8, u8) = (15, 16);
    pub const MULTIPLICATIVE: (u8, u8) = (17, 18);
    pub const CAST: u8 = 19;
    pub const PREFIX: u8 = 21;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    BitNot,
    BitXor,
    Shl,
    Shr,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    LogAnd,
    LogOr,
}

impl BinOp {
    pub fn is_comparison(self) -> bool {
        matches!(
            self,
            Self::Eq | Self::Ne | Self::Lt | Self::Le | Self::Gt | Self::Ge
        )
    }

    pub fn is_logical(self) -> bool {
        matches!(self, Self::LogAnd | Self::LogOr)
    }

    fn symbol(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Rem => "%",
            Self::BitAnd => "&",
            Self::BitOr => "|",
            Self::BitNot => "~",
            Self::BitXor => "^",
            Self::Shl => "<<",
            Self::Shr => ">>",
            Self::Eq => "==",
            Self::Ne => "!=",
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
            Self::LogAnd => "&&",
            Self::LogOr => "||",
        }
    }

    fn from_token(kind: &TokenKind) -> Option<(Self, u8, u8)> {
        let (op, (left, right)) = match kind {
            BitOperator(AND) => (Self::BitAnd, bp::BIT_AND),
            BitOperator(OR) => (Self::BitOr, bp::BIT_OR),
            BitOperator(XOR) => (Self::BitXor, bp::BIT_XOR),
            BitOperator(LSH) => (Self::Shl, bp::SHIFT),
            BitOperator(RSH) => (Self::Shr, bp::SHIFT),
            MathOperator(MathOp::Plus) => (Self::Add, bp::ADDITIVE),
            MathOperator(MathOp::Minus) => (Self::Sub, bp::ADDITIVE),
            MathOperator(MathOp::Star) => (Self::Mul, bp::MULTIPLICATIVE),
            MathOperator(MathOp::Slash) => (Self::Div, bp::MULTIPLICATIVE),
            MathOperator(MathOp::Mod) => (Self::Rem, bp::MULTIPLICATIVE),
            BinaryOperator(Cmp::Equals) => (Self::Eq, bp::COMPARE),
            BinaryOperator(Cmp::Different) => (Self::Ne, bp::COMPARE),
            BinaryOperator(Cmp::Less) => (Self::Lt, bp::COMPARE),
            BinaryOperator(Cmp::LessEqual) => (Self::Le, bp::COMPARE),
            BinaryOperator(Cmp::Greater) => (Self::Gt, bp::COMPARE),
            BinaryOperator(Cmp::GreaterEqual) => (Self::Ge, bp::COMPARE),
            And => (Self::LogAnd, bp::LOG_AND),
            Or => (Self::LogOr, bp::LOG_OR),
            _ => return None,
        };
        Some((op, left, right))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Integer(i128, CarlaType),
    String(String, CarlaType),
    Boolean(bool, CarlaType),
    Nil,
    Var(String, CarlaType),
    Pointer(String, CarlaType),
    Call(String, Vec<Expr>, CarlaType),
    CallIndirect(Box<Expr>, Vec<Expr>, CarlaType),
    Syscall(u64, Vec<Expr>, CarlaType),
    Unary(UnOp, Box<Expr>, CarlaType),
    Binary(BinOp, Box<Expr>, Box<Expr>, CarlaType),
    Cast(Box<Expr>, CarlaType),
    Deref(Box<Expr>, CarlaType),
    Reg(Loc, CarlaType),
    RegSlice(Loc, Slice, CarlaType),
    RegSliceGet(Loc, Slice, CarlaType),
    TempRef(Box<Expr>, CarlaType),
    Pack(Vec<Expr>, CarlaType),
}

impl Expr {
    pub fn ty(&self) -> CarlaType {
        match self {
            Expr::Nil => CarlaType::untyped_ptr(),
            Expr::Integer(_, t)
            | Expr::String(_, t)
            | Expr::Boolean(_, t)
            | Expr::Var(_, t)
            | Expr::Pointer(_, t)
            | Expr::Call(_, _, t)
            | Expr::CallIndirect(_, _, t)
            | Expr::Syscall(_, _, t)
            | Expr::Unary(_, _, t)
            | Expr::Binary(_, _, _, t)
            | Expr::Cast(_, t)
            | Expr::Deref(_, t)
            | Expr::Reg(_, t)
            | Expr::RegSlice(_, _, t)
            | Expr::RegSliceGet(_, _, t)
            | Expr::TempRef(_, t)
            | Expr::Pack(_, t) => t.clone(),
        }
    }

    pub fn is_const(&self) -> bool {
        matches!(
            self,
            Expr::Integer(..) | Expr::String(..) | Expr::Boolean(..) | Expr::Nil
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Compiled {
    Const(Expr),
    Lazy(ExprIterator),
}

impl Compiled {
    pub fn expr(&self) -> &Expr {
        match self {
            Compiled::Const(e) => e,
            Compiled::Lazy(it) => it.source(),
        }
    }

    pub fn from_expr(expr: Expr) -> Self {
        if expr.is_const() {
            Compiled::Const(expr)
        } else {
            Compiled::Lazy(ExprIterator::new(expr))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprId(pub [u64; 4]);

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

impl ExprId {
    fn fresh() -> Self {
        ExprId([NEXT_ID.fetch_add(1, Ordering::Relaxed), 0, 0, 0])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Const(Expr),
    Var(String),
    Temp(ExprId),
}

#[derive(Debug, Clone, PartialEq)]
pub enum StepOp {
    AddrOf(String),
    Unary(UnOp, Operand),
    Binary(BinOp, Operand, Operand),
    Call(String, Vec<Operand>),
    CallIndirect(Operand, Vec<Operand>),
    Syscall(u64, Vec<Operand>),
    Cast(Operand, CarlaType),
    Deref(Operand),
    AddrOfTemp(Operand),
    RegRead(Loc),
    RegSliceRead(Loc, Slice),
    Logical(BinOp, Operand, Vec<Step>, Operand),
    Pack(Vec<Operand>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    pub id: ExprId,
    pub ty: CarlaType,
    pub op: StepOp,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprIterator {
    source: Expr,
    steps: Vec<Step>,
    pos: usize,
    result: Operand,
}

impl ExprIterator {
    pub fn new(expr: Expr) -> Self {
        let mut steps = vec![];
        let result = lower(&expr, &mut steps);
        Self {
            source: expr,
            steps,
            pos: 0,
            result,
        }
    }

    pub fn source(&self) -> &Expr {
        &self.source
    }

    pub fn result(&self) -> &Operand {
        &self.result
    }
}

impl Iterator for ExprIterator {
    type Item = Step;

    fn next(&mut self) -> Option<Step> {
        let step = self.steps.get(self.pos)?.clone();
        self.pos += 1;
        Some(step)
    }
}

fn slice_value_bits(l: &Loc, s: &Slice) -> u32 {
    if let Some(lane) = &l.lane {
        lane.ty.bits
    } else if l.bits <= 128 {
        l.bits
    } else {
        s.width.next_power_of_two().max(8)
    }
}

fn emit(out: &mut Vec<Step>, ty: &CarlaType, op: StepOp) -> Operand {
    let id = ExprId::fresh();
    out.push(Step {
        id,
        ty: ty.clone(),
        op,
    });
    Operand::Temp(id)
}

fn lower_all(exprs: &[Expr], out: &mut Vec<Step>) -> Vec<Operand> {
    exprs.iter().map(|e| lower(e, out)).collect()
}

fn lower(e: &Expr, out: &mut Vec<Step>) -> Operand {
    match e {
        Expr::Var(n, _) => Operand::Var(n.clone()),
        Expr::Pointer(n, t) => emit(out, t, StepOp::AddrOf(n.clone())),
        Expr::Unary(op, x, t) => {
            let a = lower(x, out);
            emit(out, t, StepOp::Unary(*op, a))
        }
        Expr::Binary(op, l, r, t) if op.is_logical() => {
            let a = lower(l, out);
            let mut rhs_steps = vec![];
            let b = lower(r, &mut rhs_steps);
            emit(out, t, StepOp::Logical(*op, a, rhs_steps, b))
        }
        Expr::Binary(op, l, r, t) => {
            let a = lower(l, out);
            let b = lower(r, out);
            emit(out, t, StepOp::Binary(*op, a, b))
        }
        Expr::Call(n, args, t) => {
            let a = lower_all(args, out);
            emit(out, t, StepOp::Call(n.clone(), a))
        }
        Expr::CallIndirect(callee, args, t) => {
            let c = lower(callee, out);
            let a = lower_all(args, out);
            emit(out, t, StepOp::CallIndirect(c, a))
        }
        Expr::Syscall(n, args, t) => {
            let a = lower_all(args, out);
            emit(out, t, StepOp::Syscall(*n, a))
        }
        Expr::Cast(x, t) => {
            let a = lower(x, out);
            emit(out, t, StepOp::Cast(a, x.ty()))
        }
        Expr::Deref(x, t) => {
            let a = lower(x, out);
            emit(out, t, StepOp::Deref(a))
        }
        Expr::TempRef(x, t) => {
            let a = lower(x, out);
            emit(out, t, StepOp::AddrOfTemp(a))
        }
        Expr::Pack(items, t) => {
            let a = lower_all(items, out);
            emit(out, t, StepOp::Pack(a))
        }
        Expr::Reg(r, t) => emit(out, t, StepOp::RegRead(r.clone())),
        Expr::RegSliceGet(r, s, t) => emit(out, t, StepOp::RegSliceRead(r.clone(), *s)),
        Expr::RegSlice(..) => unreachable!("a bare reg slice is rejected by the parser"),
        literal => Operand::Const(literal.clone()),
    }
}

fn morgana_of(ty: &CarlaType) -> Option<MorganaType> {
    let untyped = ty.levels == 0 && ty.tuple.is_empty() && ty.base.is_none();
    if !ty.dim.is_empty() || untyped {
        return None;
    }
    Some(ty.to_morgana())
}

fn is_int(m: &MorganaType) -> bool {
    matches!(m, MorganaType::Integer(_) | MorganaType::UInteger(_))
}

fn is_ptr(t: &CarlaType) -> bool {
    t.levels > 0
        || match &t.base {
            Some(Base {
                symbol: BType(MorganaType::Ptr),
                ..
            }) => true,
            Some(Base {
                symbol: TypeAlias(ty),
                ..
            }) => is_ptr(ty.as_ref()),
            _ => false,
        }
}

/// `true` for the generic pointer with no pointee (what `nil` has as its type).
fn is_untyped_ptr(t: &CarlaType) -> bool {
    t.levels > 0 && t.base.is_none() && t.tuple.is_empty()
}

/// `true` when `t` is the function-reference type (`fn_ref`), compared by its own name so
/// it never gets mixed up with the generic pointer it is built on.
fn is_fn_ref(t: &CarlaType) -> bool {
    t.to_string() == DEFAULT_FN_REF
}

fn is_void(t: &CarlaType) -> bool {
    !is_ptr(t) && matches!(morgana_of(t), Some(MorganaType::Void))
}

fn is_int_ty(t: &CarlaType) -> bool {
    !is_ptr(t) && morgana_of(t).map_or(false, |m| is_int(&m))
}

fn is_signed_int(t: &CarlaType) -> bool {
    !is_ptr(t) && matches!(morgana_of(t), Some(MorganaType::Integer(_)))
}

fn is_word(t: &CarlaType) -> bool {
    !is_ptr(t)
        && matches!(
            morgana_of(t),
              Some(MorganaType::Integer(64)) | Some(MorganaType::UInteger(64))
            | Some(MorganaType::Integer(0)) | Some(MorganaType::UInteger(0))
        )
}

/// A function can only be turned into a `fn_ref` when it returns `void` or something that the
/// cast rules already treat like a pointer (a pointer or a 64-bit integer, see `cast_allowed`).
fn check_fn_ref_target(id: &str, ret: &CarlaType) -> Result<(), String> {
    if is_void(ret) || is_ptr(ret) || is_word(ret) {
        return Ok(());
    }
    Err(format!(
        "can't take `::{id}`: it returns `{}`, and only functions returning `void`, a pointer or \
         a 64-bit integer can be turned into a `fn_ref`",
        ret.to_string()
    ))
}

fn fits(v: i128, m: &MorganaType) -> bool {
    match m {
        MorganaType::Integer(b) if *b > 0 && *b < 128 => {
            let lim = 1i128 << (*b - 1);
            (-lim..lim).contains(&v)
        }
        MorganaType::UInteger(b) if *b > 0 && *b < 127 => v >= 0 && v < (1i128 << *b),
        MorganaType::UInteger(_) => v >= 0,
        _ => true,
    }
}

fn check_fits(v: i128, ty: &CarlaType) -> Result<(), String> {
    match morgana_of(ty) {
        Some(m) if !fits(v, &m) => Err(format!(
            "the value {v} doesn't fit in `{}` (compile-time overflow)",
            ty.to_string()
        )),
        _ => Ok(()),
    }
}

fn unsupported(op: impl std::fmt::Debug, tys: &[&CarlaType]) -> String {
    let names: Vec<String> = tys.iter().map(|t| format!("`{}`", t.to_string())).collect();
    format!(
        "operator {op:?} doesn't apply to {} (operator overloading isn't supported yet)",
        names.join(" and ")
    )
}

fn type_unary(op: UnOp, t: &CarlaType) -> Result<CarlaType, String> {
    match (op, morgana_of(t)) {
        (UnOp::Neg, Some(MorganaType::Integer(_))) => Ok(t.clone()),
        (UnOp::Neg, Some(MorganaType::UInteger(_))) => {
            Err(format!("can't negate `{}`: it is an unsigned type", t.to_string()))
        }
        (UnOp::Not, Some(m)) if is_int(&m) => Ok(t.clone()),
        _ => Err(unsupported(op, &[t])),
    }
}

fn type_binary(
    op: BinOp,
    l: &CarlaType,
    r: &CarlaType,
    bool_ty: &CarlaType,
) -> Result<CarlaType, String> {
    if op.is_logical() {
        let want = bool_ty.to_string();
        if l.to_string() != want || r.to_string() != want {
            return Err(format!(
                "operator `{}` needs `{want}` on both sides, found `{}` and `{}` \
                 (compare explicitly, e.g. `x != 0`)",
                op.symbol(),
                l.to_string(),
                r.to_string()
            ));
        }
        return Ok(bool_ty.clone());
    }

    if matches!(op, BinOp::Eq | BinOp::Ne) && is_ptr(l) && is_ptr(r) {
        // A `fn_ref` is only comparable with another `fn_ref` (or `nil`); anything else needs `as`.
        if is_fn_ref(l) != is_fn_ref(r) && !is_untyped_ptr(l) && !is_untyped_ptr(r) {
            return Err(format!(
                "can't compare `{}` with `{}` (a `fn_ref` only compares with another `fn_ref`; \
                 use `as`)",
                l.to_string(),
                r.to_string()
            ));
        }
        return Ok(bool_ty.clone());
    }

    let (Some(ml), Some(mr)) = (morgana_of(l), morgana_of(r)) else {
        return Err(unsupported(op, &[l, r]));
    };

    if !is_int(&ml) || !is_int(&mr) {
        return Err(unsupported(op, &[l, r]));
    }

    if ml != mr {
        return Err(format!(
            "mismatched types in {op:?}: `{}` and `{}` (no implicit conversion; use `as`)",
            l.to_string(),
            r.to_string()
        ));
    }

    Ok(if op.is_comparison() {
        bool_ty.clone()
    } else {
        l.clone()
    })
}

fn adapt(lit: Expr, to: &CarlaType) -> Result<Expr, String> {
    match (&lit, morgana_of(to)) {
        (Expr::Integer(v, _), Some(m)) if is_int(&m) => {
            check_fits(*v, to)?;
            Ok(Expr::Integer(*v, to.clone()))
        }
        _ => Ok(lit),
    }
}

fn make_unary(op: UnOp, x: Expr) -> Result<Expr, String> {
    let ty = type_unary(op, &x.ty())?;
    let folded = match (op, &x) {
        (UnOp::Neg, Expr::Integer(i, _)) => {
            let v = i.checked_neg().ok_or("compile-time overflow")?;
            check_fits(v, &ty)?;
            Some(Expr::Integer(v, ty.clone()))
        }
        (UnOp::Not, Expr::Boolean(b, _)) => Some(Expr::Boolean(!b, ty.clone())),
        (UnOp::Not, Expr::Integer(i, _)) => Some(Expr::Integer(!(*i as i128), ty.clone())),
        _ => None,
    };
    Ok(folded.unwrap_or_else(|| Expr::Unary(op, Box::new(x), ty)))
}

fn make_binary(op: BinOp, l: Expr, r: Expr, syms: &Symbols) -> Result<Expr, String> {
    if matches!(op, BinOp::Add | BinOp::Sub) {
        match (is_ptr(&l.ty()), is_ptr(&r.ty()), op) {
            (true, false, _) => return make_ptr_offset(op, l, r, syms),
            (false, true, BinOp::Add) => return make_ptr_offset(op, r, l, syms),
            (false, true, _) => return Err("can't subtract a pointer from a number".into()),
            (true, true, _) => {
                return Err(
                    "can't add or subtract two pointers (cast both to integers to get a distance)"
                        .into(),
                )
            }
            _ => {}
        }
    }

    let bool_ty = builtin(syms, DEFAULT_BOOL)?;

    let (l, r) = match (l.is_const(), r.is_const()) {
        (true, false) => (adapt(l, &r.ty())?, r),
        (false, true) => {
            let t = l.ty();
            (l, adapt(r, &t)?)
        }
        _ => (l, r),
    };

    let ty = type_binary(op, &l.ty(), &r.ty(), &bool_ty)?;
    if let Some(folded) = fold_binary(op, &l, &r, &ty)? {
        return Ok(folded);
    }
    Ok(Expr::Binary(op, Box::new(l), Box::new(r), ty))
}

fn fold_logical(op: BinOp, l: &Expr, r: &Expr, ty: &CarlaType) -> Option<Expr> {
    let and = op == BinOp::LogAnd;
    match (l, r) {
        (Expr::Boolean(a, _), Expr::Boolean(b, _)) => Some(Expr::Boolean(
            if and { *a && *b } else { *a || *b },
            ty.clone(),
        )),
        (Expr::Boolean(a, _), other) => Some(if *a == and {
            other.clone()
        } else {
            Expr::Boolean(*a, ty.clone())
        }),
        (other, Expr::Boolean(b, _)) if *b == and => Some(other.clone()),
        _ => None,
    }
}

fn fold_binary(op: BinOp, l: &Expr, r: &Expr, ty: &CarlaType) -> Result<Option<Expr>, String> {
    use BinOp::*;

    if op.is_logical() {
        return Ok(fold_logical(op, l, r, ty));
    }

    if let (Expr::Boolean(a, _), Expr::Boolean(b, _)) = (l, r) {
        return Ok(match op {
            BitAnd => Some(Expr::Boolean(a & b, ty.clone())),
            BitOr => Some(Expr::Boolean(a | b, ty.clone())),
            BitXor => Some(Expr::Boolean(a ^ b, ty.clone())),
            Eq => Some(Expr::Boolean(a == b, ty.clone())),
            Ne => Some(Expr::Boolean(a != b, ty.clone())),
            _ => None,
        });
    }

    let (Expr::Integer(a, _), Expr::Integer(b, _)) = (l, r) else {
        return Ok(None);
    };

    if op.is_comparison() {
        let v = match op {
            Eq => a == b,
            Ne => a != b,
            Lt => a < b,
            Le => a <= b,
            Gt => a > b,
            Ge => a >= b,
            _ => unreachable!(),
        };
        return Ok(Some(Expr::Boolean(v, ty.clone())));
    }

    let v = match op {
        Div | Rem if *b == 0 => return Err("compile-time division by zero".into()),
        Add => a.checked_add(*b),
        Sub => a.checked_sub(*b),
        Mul => a.checked_mul(*b),
        Div => a.checked_div(*b),
        Rem => a.checked_rem(*b),
        BitAnd => Some(a & b),
        BitOr => Some(a | b),
        BitXor => Some(a ^ b),
        Shl | Shr if !(0..=126).contains(b) => {
            return Err(format!("the shift amount {b} is out of range (0 to 126)"))
        }
        Shl => {
            let shifted = a.wrapping_shl(*b as u32);
            if (shifted >> *b) == *a {
                Some(shifted)
            } else {
                None
            }
        }
        Shr => Some(a >> *b),
        _ => unreachable!("comparison and logical operators are handled above"),
    }
    .ok_or("compile-time overflow")?;

    check_fits(v, ty)?;
    Ok(Some(Expr::Integer(v, ty.clone())))
}

fn cast_allowed(from: &CarlaType, to: &CarlaType) -> bool {
    let (from_ptr, to_ptr) = (is_ptr(from), is_ptr(to));

    (is_int_ty(from) && is_int_ty(to))
        || (from_ptr && to_ptr)
        || (from_ptr && is_word(to))
        || (is_word(from) && to_ptr)
}

fn literal_int(e: &Expr) -> Option<i128> {
    match e {
        Expr::Integer(v, _) => Some(*v),
        Expr::Boolean(b, _) => Some(*b as i128),
        _ => None,
    }
}

fn make_cast(x: Expr, to: CarlaType) -> Result<Expr, String> {
    let from = x.ty();
    let (from_name, to_name) = (from.to_string(), to.to_string());

    if from_name == to_name {
        return Ok(x);
    }

    if !cast_allowed(&from, &to) {
        return Err(format!(
            "can't convert `{from_name}` to `{to_name}` (only integer <-> integer, pointer <-> pointer and pointer <-> 64-bit integer are allowed)"
        ));
    }

    if is_int_ty(&to) {
        if let Some(v) = literal_int(&x) {
            check_fits(v, &to)?;
            return Ok(Expr::Integer(v, to));
        }
    }

    Ok(Expr::Cast(Box::new(x), to))
}

fn as_pointer(ty: &CarlaType) -> Option<CarlaType> {
    if ty.levels > 0 {
        return Some(ty.clone());
    }

    match &ty.base {
        Some(Base {
            symbol: TypeAlias(inner),
            ..
        }) if ty.dim.is_empty() => as_pointer(inner),
        _ => None,
    }
}

fn make_deref(x: Expr) -> Result<Expr, String> {
    let ty = x.ty();

    if is_fn_ref(&ty) {
        return Err(
            "cannot dereference a `fn_ref`; call it with `fn name(...)` (or cast it first, e.g. `as T*`)"
                .into(),
        );
    }

    let Some(mut pointee) = as_pointer(&ty) else {
        return Err(if is_ptr(&ty) {
            format!(
                "cannot dereference `{}`: it is an untyped pointer (cast it first, e.g. `as T*`)",
                ty.to_string()
            )
        } else {
            format!("cannot dereference `{}`: it is not a pointer", ty.to_string())
        });
    };
    pointee.levels -= 1;

    if pointee.levels == 0 && pointee.base.is_none() && pointee.tuple.is_empty() {
        return Err("cannot dereference an untyped pointer (cast it first, e.g. `as T*`)".into());
    }

    if pointee.levels == 0 && !pointee.dim.is_empty() {
        return Err("dereferencing a pointer to an array isn't supported yet".into());
    }

    Ok(Expr::Deref(Box::new(x), pointee))
}

/// Stride in bytes of one element of type `t`, used to scale `ptr + n` / `ptr - n` / `ptr[n]`.
///
/// It is exactly `sizeof(t)` (see `layout`), so it works for integers, pointers, tuples, aliases
/// (`string`, ...) and arrays alike. A tuple's size is already rounded up to its own alignment,
/// which is what makes consecutive elements line up (and match the `[]T` variadic lists).
fn elem_size(t: &CarlaType) -> Result<i128, String> {
    if is_ptr(t) {
        return Ok(8);
    }

    let (size, _) = layout(t).map_err(|e| {
        format!(
            "pointer arithmetic over `{}` isn't possible: {e}",
            t.to_string()
        )
    })?;

    if size == 0 {
        return Err(format!(
            "pointer arithmetic over `{}` isn't possible: its size is 0",
            t.to_string()
        ));
    }

    Ok(size)
}

fn make_ptr_offset(op: BinOp, ptr: Expr, off: Expr, syms: &Symbols) -> Result<Expr, String> {
    let pty = ptr.ty();

    if is_fn_ref(&pty) {
        return Err(
            "pointer arithmetic on a `fn_ref` isn't allowed (a function address has no elements)"
                .into(),
        );
    }

    if matches!(ptr, Expr::Pointer(..)) {
        return Err(
            "pointer arithmetic on `&variable` isn't supported yet; keep the pointer in a variable first"
                .into(),
        );
    }

    let Some(mut pointee) = as_pointer(&pty) else {
        return Err(format!(
            "pointer arithmetic needs a typed pointer, found `{}` (cast it first, e.g. `as T*`)",
            pty.to_string()
        ));
    };
    pointee.levels -= 1;
    let size = elem_size(&pointee)?;

    let word = builtin(syms, DEFAULT_SYSCALL_RET)?;
    let off = if off.is_const() { adapt(off, &word)? } else { off };

    let off_ty = off.ty();
    if !is_word(&off_ty) {
        return Err(format!(
            "a pointer offset must be a 64-bit integer, found `{}` (use `as`)",
            off_ty.to_string()
        ));
    }

    let scaled = if size == 1 {
        off
    } else {
        make_binary(BinOp::Mul, off, Expr::Integer(size, off_ty), syms)?
    };

    Ok(Expr::Binary(op, Box::new(ptr), Box::new(scaled), pty))
}

fn align_up(v: i128, a: i128) -> i128 {
    (v + a - 1) / a * a
}

/// (size, alignment) in bytes of a primitive.
fn layout_morgana(m: &MorganaType) -> Result<(i128, i128), String> {
    match m {
        MorganaType::Integer(b) | MorganaType::UInteger(b) if *b > 0 && *b % 8 == 0 => {
            let size = (*b / 8) as i128;
            Ok((size, size.min(8)))
        }
        MorganaType::Integer(b) | MorganaType::UInteger(b) => {
            Err(format!("a {b}-bit integer doesn't have a whole number of bytes"))
        }
        MorganaType::Ptr => Ok((8, 8)),
        MorganaType::Void => Err("`void` has no size".into()),
        MorganaType::Tuple(items) => {
            let (mut offset, mut max) = (0i128, 1i128);
            for item in items {
                let (s, a) = layout_morgana(item)?;
                offset = align_up(offset, a) + s;
                max = max.max(a);
            }
            Ok((align_up(offset, max), max))
        }
    }
}

fn layout(t: &CarlaType) -> Result<(i128, i128), String> {
    if t.levels > 0 {
        return Ok((8, 8));
    }

    let (size, align) = if !t.tuple.is_empty() {
        let (mut offset, mut max) = (0i128, 1i128);
        for item in &t.tuple {
            let (s, a) = layout(item)?;
            offset = align_up(offset, a)
                .checked_add(s)
                .ok_or("compile-time overflow in sizeof")?;
            max = max.max(a);
        }
        (align_up(offset, max), max)
    } else if let Some(base) = &t.base {
        match &base.symbol {
            BType(m) => layout_morgana(m)?,
            TypeAlias(alias) => layout(alias.as_ref())?,
            _ => return Err(format!("`{}` isn't a type", t.to_string())),
        }
    } else {
        return Err(format!("`{}` has no size", t.to_string()));
    };

    let mut total = size;
    for dim in &t.dim {
        if *dim < 0 {
            return Err(format!("invalid array dimension in `{}`", t.to_string()));
        }
        total = total
            .checked_mul(*dim)
            .ok_or("compile-time overflow in sizeof")?;
    }

    Ok((total, align))
}

/// Byte offset of every element of a tuple, with the same layout rule `sizeof` uses
/// (each element aligned to its own alignment, in declaration order).
/// Destructuring reads element `i` at `&tuple + offsets[i]`.
pub fn tuple_offsets(items: &[CarlaType]) -> Result<Vec<i128>, String> {
    let mut offsets = Vec::with_capacity(items.len());
    let mut offset = 0i128;

    for item in items {
        let (size, align) = layout(item)?;
        offset = align_up(offset, align);
        offsets.push(offset);
        offset += size;
    }

    Ok(offsets)
}

fn make_index(base: Expr, index: Expr, syms: &Symbols) -> Result<Expr, String> {
    let ty = base.ty();

    if !ty.dim.is_empty() {
        return Err(format!("indexing the array `{}` isn't supported yet", ty.to_string()));
    }
    if as_pointer(&ty).is_none() {
        return Err(format!("cannot index `{}`: it is not a pointer", ty.to_string()));
    }

    make_deref(make_ptr_offset(BinOp::Add, base, index, syms)?)
}

/// Builds the `self` argument of an extended call: always a POINTER to the receiver.
///
/// * a variable          -> `&variable`
/// * `p.*` / `p[i]`      -> `p` (nothing is copied, `f` works on the real object)
/// * any other value     -> only with `by_temp` (the `::` operator): the value is materialized
///   in a temporary slot and `self` is the address of that slot, never the value itself.
fn self_ref(receiver: Expr, by_temp: bool) -> Result<Expr, String> {
    match receiver {
        Expr::Var(name, ty) => Ok(Expr::Pointer(name, ty.pointer_to())),
        // `p.*.f()` / `p[i].f()`: `&(*p)` is just `p`. Nothing is copied, so `f` works on the
        // real object (a `let x := p.*` would work on a copy of it).
        Expr::Deref(ptr, _) => Ok(*ptr),
        Expr::RegSlice(..) => Err(BARE_SLICE.into()),
        other if by_temp => {
            let ty = other.ty();
            if is_void(&ty) {
                return Err("can't use a `void` value as `self`".into());
            }
            // `self` is a pointer to the temporary, never the value itself
            Ok(Expr::TempRef(Box::new(other), ty.pointer_to()))
        }
        other => Err(format!(
            "cannot take a reference to a temporary value of type `{}` to use as `self` with `.`; \
             call it with `::` (e.g. `value::f()`), store it in a variable first, \
             or call it on a pointer: `p.*.f()`",
            other.ty().to_string()
        )),
    }
}

pub fn can_make_ptr(sym: &Symbol) -> bool {
    matches!(sym, Variable(_, _, _) | ExternFunction(_, _))
}

fn builtin(syms: &Symbols, id: &str) -> Result<CarlaType, String> {
    CarlaType::named(id, syms).ok_or_else(|| format!("type '{id}' isn't registered in Symbols"))
}

fn symbol_ty(id: &str, sym: &Symbol) -> Result<CarlaType, String> {
    match sym {
        Variable(Some(t), _, _) => Ok((**t).clone()),
        Variable(None, _, _) => Err(format!("variable '{id}' doesn't have a type yet")),
        ExternFunction(ret, _) | UnfoldFunction(ret, _) => Ok((**ret).clone()),
        _ => Err(format!("'{id}' isn't a value")),
    }
}

fn pointer_ty(id: &str, sym: &Symbol) -> Result<CarlaType, String> {
    match sym {
        Variable(Some(t), _, false) => Ok(t.pointer_to()),
        Variable(None, _, false) => Err(format!("variable '{id}' doesn't have a type yet")),
        _ => Ok(CarlaType::untyped_ptr()),
    }
}

fn int_literal(v: i128, syms: &Symbols) -> Result<Expr, String> {
    let ty = builtin(syms, DEFAULT_INT)?;
    check_fits(v, &ty)?;
    Ok(Expr::Integer(v, ty))
}

// ---------------------------------------------------------------------------------------------
// Variadic calls
//
// A function whose last parameter is `[]T` / `[]any` takes any number (zero included) of extra
// arguments. At the call site they are packed into the tuple the parameter really is:
//
//   []T    ->  |ulong, T*|                       count, pointer to the list of T
//   []any  ->  |ulong, string*, generic_ptr|     count, pointer to the type names, pointer to
//                                                the values
//
// Everything lives on the CALLER's stack (a temporary slot that lasts until the end of the
// statement), so nothing is allocated and nothing has to be freed.
//
// Layout of the lists:
//   * `[]T`:   n values of `T`, one after the other (stride = sizeof(T)).
//   * `[]any`: n `string`s with the type name of each value (`int`, `byte*`, `string`, ...) and
//              n slots of 8 bytes, one per value, in the same order. A pointer or a 64-bit
//              integer is stored as it is, a smaller integer is widened to 64 bits (sign- or
//              zero-extended by its own signedness) and anything bigger (a `string`, a tuple)
//              is stored as a POINTER to a copy of it. So `list[i]` is at `8 * i`.
// ---------------------------------------------------------------------------------------------

thread_local! {
    /// function name (the same one pushed into `Symbols`) -> (fixed parameters, variadic kind)
    static VARIADICS: RefCell<HashMap<String, (usize, Variadic)>> = RefCell::new(HashMap::new());
}

/// Tells the call parser how the function `name` takes its arguments.
///
/// Call it where a function is declared, with the same `name` it pushes into `Symbols`
/// (the mangled `abi::extension_symbol(..)` name for extended functions) and its parsed
/// parameters: `register_signature(&name, &lambda.args)`. A function without a variadic
/// parameter clears any previous entry, so calling it for every function is fine.
pub fn register_signature(name: &str, params: &[Arg]) {
    let variadic = params.last().and_then(|a| a.variadic.clone());
    let fixed: usize = params
        .iter()
        .filter(|a| a.variadic.is_none())
        .map(|a| a.ids.len())
        .sum();

    VARIADICS.with(|v| {
        let mut v = v.borrow_mut();
        match variadic {
            Some(kind) => {
                v.insert(name.to_string(), (fixed, kind));
            }
            None => {
                v.remove(name);
            }
        }
    });
}

fn variadic_signature(name: &str) -> Option<(usize, Variadic)> {
    VARIADICS.with(|v| v.borrow().get(name).cloned())
}

fn tuple_of(items: Vec<CarlaType>) -> CarlaType {
    CarlaType {
        tuple: items,
        ..Default::default()
    }
}

fn count_of(n: usize, syms: &Symbols) -> Result<Expr, String> {
    Ok(Expr::Integer(n as i128, builtin(syms, DEFAULT_COUNT)?))
}

/// Packs `items` (a tuple of type `list_ty`) into a stack slot and yields its address.
fn stack_list(items: Vec<Expr>, list_ty: CarlaType, ptr_ty: CarlaType) -> Expr {
    Expr::TempRef(Box::new(Expr::Pack(items, list_ty)), ptr_ty)
}

/// The extra arguments of a call to a variadic function become its last argument.
/// Functions that aren't variadic are left alone.
fn pack_variadic(syms: &Symbols, id: &str, mut args: Vec<Expr>) -> Result<Vec<Expr>, String> {
    let Some((fixed, kind)) = variadic_signature(id) else {
        return Ok(args);
    };

    if args.len() < fixed {
        return Err(format!(
            "`{id}` takes at least {fixed} argument(s) before its variadic part, found {}",
            args.len()
        ));
    }

    let extra = args.split_off(fixed);

    let packed = match kind {
        Variadic::Typed(elem) => pack_typed(syms, id, &elem, extra)?,
        Variadic::Any => pack_any(syms, id, extra)?,
    };

    args.push(packed);
    Ok(args)
}

/// `[]T`: every extra argument has to be exactly a `T` (same rule as an assignment: integer
/// literals adapt to `T`, `nil` goes into a pointer, anything else needs `as`).
fn pack_typed(
    syms: &Symbols,
    id: &str,
    elem: &CarlaType,
    extra: Vec<Expr>,
) -> Result<Expr, String> {
    if !elem.dim.is_empty() || is_void(elem) {
        return Err(format!(
            "`{id}`: `[]{}` isn't a valid variadic parameter (the element can't be `void` or an array)",
            elem.to_string()
        ));
    }

    let header = CarlaType::variadic_of(elem, syms)?;
    let n = extra.len();

    let mut items = Vec::with_capacity(n);
    for (i, value) in extra.into_iter().enumerate() {
        let value = adapt(value, elem)?;

        let nil_in_ptr = matches!(value, Expr::Nil) && is_ptr(elem);
        let (want, got) = (elem.to_string(), value.ty().to_string());

        if want != got && !nil_in_ptr {
            return Err(format!(
                "variadic argument {} of `{id}` must be a `{want}`, found `{got}` \
                 (no implicit conversion; use `as`)",
                i + 1
            ));
        }

        items.push(value);
    }

    let list = if n == 0 {
        Expr::Nil
    } else {
        stack_list(items, tuple_of(vec![elem.clone(); n]), elem.pointer_to())
    };

    Ok(Expr::Pack(vec![count_of(n, syms)?, list], header))
}

/// `[]any`: accepts every value, and remembers the type of each one by name.
fn pack_any(syms: &Symbols, id: &str, extra: Vec<Expr>) -> Result<Expr, String> {
    let header = CarlaType::variadic_dyn(syms)?;
    let n = extra.len();

    if n == 0 {
        return Ok(Expr::Pack(
            vec![count_of(0, syms)?, Expr::Nil, Expr::Nil],
            header,
        ));
    }

    let string_ty = builtin(syms, DEFAULT_STRING)?;

    let mut names = Vec::with_capacity(n);
    let mut slots = Vec::with_capacity(n);
    let mut slot_tys = Vec::with_capacity(n);

    for (i, value) in extra.into_iter().enumerate() {
        if matches!(value, Expr::Nil) {
            return Err(format!(
                "variadic argument {} of `{id}` is a bare `nil`, which has no type for `[]any` \
                 (cast it, e.g. `nil as byte*`)",
                i + 1
            ));
        }

        let ty = value.ty();
        if is_void(&ty) || !ty.dim.is_empty() {
            return Err(format!(
                "variadic argument {} of `{id}` is a `{}`, which can't go in a `[]any`",
                i + 1,
                ty.to_string()
            ));
        }

        names.push(type_name(syms, &ty)?);

        let (slot, slot_ty) = any_slot(syms, value)?;
        slots.push(slot);
        slot_tys.push(slot_ty);
    }

    let names_list = stack_list(
        names,
        tuple_of(vec![string_ty.clone(); n]),
        string_ty.pointer_to(),
    );
    let values_list = stack_list(slots, tuple_of(slot_tys), CarlaType::from_basic("generic_ptr"));

    Ok(Expr::Pack(
        vec![count_of(n, syms)?, names_list, values_list],
        header,
    ))
}

/// The 8-byte slot a value takes inside a `[]any` list (see the layout above).
fn any_slot(syms: &Symbols, value: Expr) -> Result<(Expr, CarlaType), String> {
    let ty = value.ty();

    if is_ptr(&ty) || is_word(&ty) {
        return Ok((value, ty));
    }

    if is_int_ty(&ty) {
        let wide = reg::int_type(syms, 64, is_signed_int(&ty))?;
        return Ok((make_cast(value, wide.clone())?, wide));
    }

    let slot = ty.pointer_to();
    Ok((Expr::TempRef(Box::new(value), slot.clone()), slot))
}

/// `"<type name>"::to_string()`: the `string` that names the type of a `[]any` value.
fn type_name(syms: &Symbols, ty: &CarlaType) -> Result<Expr, String> {
    let cstrz = builtin(syms, DEFAULT_CSTRZ)?;
    let full = abi::extension_symbol(&cstrz, DEFAULT_TO_STRING);

    let Some(sym) = syms.lookup(&full) else {
        return Err(format!(
            "`[]any` needs `{}.{DEFAULT_TO_STRING}` in scope to name the type of each argument \
             (import the strings module)",
            cstrz.to_string()
        ));
    };

    let ret = symbol_ty(&full, sym)?;
    let receiver = self_ref(Expr::String(ty.to_string(), cstrz), true)?;

    Ok(Expr::Call(full, vec![receiver], ret))
}

pub fn starts_with(kind: &TokenKind) -> bool {
    matches!(
        kind,
        Identifier(_)
            | Keyword(Syscall)
            | Keyword(Reg)
            | Keyword(RegOk)
            | Keyword(FnKw)
            | DoubleColon
            | Dot
            | Bool(_)
            | BitOperator(AND)
            | Integer(_)
            | Hexa(_)
            | Octal(_)
            | Float(_)
            | Str(_)
            | MathOperator(Minus)
            | Bang
            | Nil
    )
}

struct Parser<'a, 'c> {
    syms: &'a mut Symbols,
    it: &'a mut ContextIterator<'c>,
}

impl<'a, 'c> Parser<'a, 'c> {
    fn new(syms: &'a mut Symbols, it: &'a mut ContextIterator<'c>) -> Self {
        Self { syms, it }
    }

    fn at(&mut self, kind: &TokenKind) -> bool {
        matches!(self.it.peek(), Some(Common(tk)) if tk.kind == *kind)
    }

    fn at_bracket(&mut self) -> bool {
        matches!(self.it.peek(), Some(Block(Bracket, _)))
    }

    fn peek_binary_op(&mut self) -> Option<(BinOp, u8, u8)> {
        match self.it.peek() {
            Some(Common(tk)) => BinOp::from_token(&tk.kind),
            _ => None,
        }
    }

    fn sizeof(&mut self) -> Result<Expr, String> {
        let err = "Expected a type between parentheses after `sizeof`, e.g. `sizeof(int)`";

        let Some(Block(Paren, ctx)) = self.it.next() else {
            return Err(err.into());
        };

        let mut sub = ContextIterator::new(ctx.as_slice(), 0);
        let Some(Common(first)) = sub.next() else {
            return Err(err.into());
        };

        let Node::InternalType(ty) = types::work(first, self.syms, &mut sub)? else {
            return Err(err.into());
        };

        if sub.remaining() > 0 {
            return Err("leftover tokens inside `sizeof(...)`".into());
        }

        let (size, _) = layout(&ty)?;
        int_literal(size, self.syms)
    }

    fn next_token(&mut self) -> Option<&'c Token> {
        match self.it.next() {
            Some(Common(tk)) => Some(tk),
            _ => None,
        }
    }

    fn next_identifier(&mut self) -> Option<&'c str> {
        match &self.next_token()?.kind {
            Identifier(id) => Some(id.as_str()),
            _ => None,
        }
    }

    fn expr(&mut self, min_bp: u8) -> Result<Expr, String> {
        let head = match self.it.next() {
            None => return Err("incomplete expression".into()),
            Some(Block(Paren, inner)) => self.group(inner)?,
            Some(Common(tk)) => self.prefix(tk)?,
            Some(_) => return Err("unexpected block at the start of an expression".into()),
        };
        self.continue_expr(head, min_bp)
    }

    fn continue_expr(&mut self, head: Expr, min_bp: u8) -> Result<Expr, String> {
        let lhs = self.postfix(head)?;
        if matches!(lhs, Expr::RegSlice(..)) {
            return Err(BARE_SLICE.into());
        }
        self.climb(lhs, min_bp)
    }

    fn group(&mut self, inner: &[Context]) -> Result<Expr, String> {
        let mut sub = ContextIterator::new(inner, 0);
        let value = Parser::new(self.syms, &mut sub).expr(0)?;
        if sub.remaining() > 0 {
            return Err("leftover tokens inside the parentheses".into());
        }
        Ok(value)
    }

    fn climb(&mut self, mut lhs: Expr, min_bp: u8) -> Result<Expr, String> {
        loop {
            if self.at(&Keyword(As)) {
                if bp::CAST < min_bp {
                    break;
                }
                self.it.next();
                lhs = self.cast(lhs)?;
                lhs = self.postfix(lhs)?;
                continue;
            }

            let Some((op, left, right)) = self.peek_binary_op() else {
                break;
            };
            if left < min_bp {
                break;
            }
            self.it.next();
            let rhs = self.expr(right)?;
            lhs = make_binary(op, lhs, rhs, self.syms)?;

            if op.is_comparison()
                && matches!(self.peek_binary_op(), Some((next, _, _)) if next.is_comparison())
            {
                return Err("comparison operators can't be chained (use `a < b && b < c`)".into());
            }
        }
        Ok(lhs)
    }

    fn cast(&mut self, value: Expr) -> Result<Expr, String> {
        let Some(first) = self.next_token() else {
            return Err("Expected a type after `as`".into());
        };
        let target = types::parse_path(first, self.syms, self.it)?;
        make_cast(value, target)
    }

    /// Postfix:
    ///   `value.*`              reads the value behind a pointer
    ///   `value[i]`             reads element `i` (same as `(value + i).*`)
    ///   `value.function(args)` calls an extended function, passing `&value` as `self`
    ///   `value::function(args)` like `.`, but it also accepts a temporary as the receiver
    ///                           (`"text"::print()`): `self` is a pointer to a temporary slot
    ///                           that lives until the end of the statement, never the value itself
    fn postfix(&mut self, mut lhs: Expr) -> Result<Expr, String> {
        loop {
            if self.at(&Dot) {
                self.it.next();

                if self.at(&MathOperator(MathOp::Star)) {
                    self.it.next();
                    lhs = match lhs {
                        Expr::RegSlice(r, s, t) => Expr::RegSliceGet(r, s, t),
                        Expr::RegSliceGet(r, s, t) if r.is_whole_lane(&s) => {
                            Expr::RegSliceGet(r, s, t)
                        }
                        Expr::Reg(..) => {
                            return Err("`reg[..]` without a slice is already a value; `.*` only \
                                        reads a slice (`reg[reg, ax, 64, 0..3].*`)"
                                .into())
                        }
                        other => make_deref(other)?,
                    };
                    continue;
                }

                let Some(member) = self.next_identifier() else {
                    return Err("Expected a function name or `*` after '.'".into());
                };
                let owner = lhs.ty();
                lhs = self.extended_call(&owner, Some(lhs), member, false)?;
                continue;
            }

            if self.at(&DoubleColon) {
                self.it.next();

                let Some(member) = self.next_identifier() else {
                    return Err(
                        "Expected a function name after `::` (e.g. `\"text\"::print()`)".into(),
                    );
                };
                let owner = lhs.ty();
                lhs = self.extended_call(&owner, Some(lhs), member, true)?;
                continue;
            }

            if self.at_bracket() {
                let Some(Block(_, inner)) = self.it.next() else {
                    unreachable!("at_bracket just saw a block");
                };
                let index = self.group(inner)?;
                lhs = make_index(lhs, index, self.syms)?;
                continue;
            }

            break;
        }
        Ok(lhs)
    }

    fn prefix(&mut self, token: &Token) -> Result<Expr, String> {
        match &token.kind {
            Keyword(Syscall) => self.syscall(),
            Keyword(Reg) => self.reg(false),
            Keyword(RegOk) => self.reg(true),

            DoubleColon => self.fn_address(),
            Keyword(FnKw) => self.fn_call(),

            MathOperator(Minus) => make_unary(UnOp::Neg, self.expr(bp::PREFIX)?),
            Bang => make_unary(UnOp::Not, self.expr(bp::PREFIX)?),
            BitOperator(AND) => self.address_of(),

            Integer(i) | Hexa(i) | Octal(i) => int_literal(*i, self.syms),
            Bool(b) => Ok(Expr::Boolean(*b, builtin(self.syms, DEFAULT_BOOL)?)),
            Str(s) => Ok(Expr::String(
                s.clone(),
                builtin(self.syms, DEFAULT_CSTRZ)?,
            )),
            Nil => Ok(Expr::Nil),
            Float(_) => Err("Morgana doesn't have a float type yet".into()),

            Identifier(id) => self.identifier(token, id),

            Dot => match self.it.next() {
                    Some(Block(Paren, inner)) => self.group(inner),
                    _ => Err("Expected `(` after `.` at the start of an expression (e.g. `.(1 + 3) * 2`)".into()),
            },

            _ => Err(format!(
                "unsupported token in expression: {:?}",
                token.kind
            )),
        }
    }

    fn address_of(&mut self) -> Result<Expr, String> {
        let Some(id) = self.next_identifier() else {
            return Err("Expected an identifier after a & (pointer operator)".into());
        };
        let Some(sym) = self.syms.lookup(id) else {
            return Err(format!("{id} was not found"));
        };
        if !can_make_ptr(sym) {
            return Err(format!("you can't make a pointer of {id}"));
        }
        let ty = pointer_ty(id, sym)?;
        Ok(Expr::Pointer(id.to_string(), ty))
    }

    fn fn_address(&mut self) -> Result<Expr, String> {
        let err = "Expected a function name after `::` (e.g. `::print`)";
        let Some(id) = self.next_identifier() else {
            return Err(err.into());
        };
        let Some(sym) = self.syms.lookup(id) else {
            return Err(format!("unknown symbol: '{id}'"));
        };

        match sym {
            Variable(_, _, true) | ExternFunction(..) => {}
            UnfoldFunction(..) => {
                return Err(format!(
                    "can't take `::{id}`: `unfold` functions are expanded in place and have no address"
                ))
            }
            _ => return Err(format!("can't take `::{id}`: '{id}' isn't a function")),
        }

        let ret = symbol_ty(id, sym)?;
        check_fn_ref_target(id, &ret)?;

        let ty = builtin(self.syms, DEFAULT_FN_REF)?;
        Ok(Expr::Pointer(id.to_string(), ty))
    }

    fn fn_call(&mut self) -> Result<Expr, String> {
        let err = "Expected a `fn_ref` variable after `fn` (e.g. `fn callback(...)`)";
        let Some(id) = self.next_identifier() else {
            return Err(err.into());
        };
        let Some(sym) = self.syms.lookup(id) else {
            return Err(format!("unknown symbol: '{id}'"));
        };

        let callee = match sym {
            Variable(Some(t), _, false) => Expr::Var(id.to_string(), (**t).clone()),
            Variable(None, _, false) => {
                return Err(format!("variable '{id}' doesn't have a type yet"))
            }
            _ => return Err(err.into()),
        };

        if !is_fn_ref(&callee.ty()) {
            return Err(format!(
                "`fn {id}(...)` needs a `{DEFAULT_FN_REF}`, but '{id}' is a `{}` \
                 (get one with `::name`)",
                callee.ty().to_string()
            ));
        }

        let args = self.arguments(&format!("fn {id}"))?;
        let ret = builtin(self.syms, DEFAULT_VOID)?;
        Ok(Expr::CallIndirect(Box::new(callee), args, ret))
    }

    fn identifier(&mut self, token: &Token, id: &str) -> Result<Expr, String> {
        if id == "sizeof" {
            return self.sizeof();
        }

        let Some(sym) = self.syms.lookup(id) else {
            return Err(format!("unknown symbol: '{id}'"));
        };

        if matches!(sym, BType(_) | TypeAlias(_)) {
            return self.type_call(token);
        }

        let is_call = match sym {
            Variable(_, _, func) => *func,
            ExternFunction(..) | UnfoldFunction(..) => true,
            _ => return Err(format!("{id} isn't a valid identifier")),
        };
        let ty = symbol_ty(id, sym)?;

        if is_call {
            self.call(id, ty)
        } else {
            Ok(Expr::Var(id.to_string(), ty))
        }
    }

    fn type_call(&mut self, token: &Token) -> Result<Expr, String> {
        let owner = types::parse_path(token, self.syms, self.it)?;
        let err = || {
            format!(
                "`{0}` is a type, not a value; use `{0}.function(...)`",
                owner.to_string()
            )
        };

        let Some(dot) = self.next_token() else {
            return Err(err());
        };
        if dot.kind != Dot {
            return Err(err());
        }
        let Some(member) = self.next_identifier() else {
            return Err(err());
        };

        // no receiver on this path, so `by_temp` is unused
        self.extended_call(&owner, None, member, false)
    }

    /// `by_temp`: the call was written with `::`, so a temporary receiver is accepted and
    /// passed as a pointer to a temporary slot.
    fn extended_call(
        &mut self,
        owner: &CarlaType,
        receiver: Option<Expr>,
        member: &str,
        by_temp: bool,
    ) -> Result<Expr, String> {
        let full = abi::extension_symbol(owner, member);

        let Some(sym) = self.syms.lookup(&full) else {
            return Err(format!(
                "`{}` has no extended function `{member}` ({full})",
                owner.to_string()
            ));
        };

        if !matches!(
            sym,
            Variable(_, _, true) | ExternFunction(..) | UnfoldFunction(..)
        ) {
            return Err(format!("`{full}` isn't a function"));
        }

        let ret = symbol_ty(&full, sym)?;
        let args = self.arguments(&full)?;

        let mut args = pack_variadic(self.syms, &full, args)?;

        if let Some(receiver) = receiver {
            args.insert(0, self_ref(receiver, by_temp)?);
        }

        Ok(Expr::Call(full, args, ret))
    }

    fn call(&mut self, id: &str, ret: CarlaType) -> Result<Expr, String> {
        let args = self.arguments(id)?;
        let args = pack_variadic(self.syms, id, args)?;
        Ok(Expr::Call(id.to_string(), args, ret))
    }

    fn arguments(&mut self, id: &str) -> Result<Vec<Expr>, String> {
        let err = format!("Invalid {id} call");
        let Some(Block(Paren, ctx)) = self.it.next() else {
            return Err(err);
        };

        let mut args = vec![];
        let mut sub = ContextIterator::new(ctx.as_slice(), 0);
        while sub.remaining() > 0 {
            args.push(Parser::new(self.syms, &mut sub).expr(0)?);
            match sub.next() {
                None => break,
                Some(Common(t)) if matches!(t.kind, Comma) => {}
                _ => return Err(err),
            }
        }
        Ok(args)
    }

    /// `reg[...]` / `reg_ok[...]`, see the `reg` module.
    ///
    /// The arguments are read straight from the tokens (not as expressions): `ax` is a register
    /// name, not a variable.
    fn reg(&mut self, probe: bool) -> Result<Expr, String> {
           let kw = if probe { "reg_ok" } else { "reg" };

           let Some(Block(Bracket, ctx)) = self.it.next() else {
               return Err(format!(
                   "Expected `[...]` after `{kw}`, e.g. `{kw}[reg, ax, 64]`, `{kw}[io, 0x24]` or `{kw}[vreg, xmm0, 128, u32, 2]`"
               ));
           };

           let mut sub = ContextIterator::new(ctx.as_slice(), 0);
           let mut toks: Vec<&TokenKind> = vec![];
           while let Some(c) = sub.next() {
               match c {
                   Common(tk) => toks.push(&tk.kind),
                   _ => {
                       return Err(format!(
                           "`{kw}[...]` only takes names, numbers, `,` and `..`"
                       ))
                   }
               }
           }

           let raw = reg::parse_args(&toks).map_err(|e| format!("`{kw}[...]`: {e}"))?;
           let checked = reg::validate(&raw);

           if probe {
               let bool_ty = builtin(self.syms, DEFAULT_BOOL)?;
               return Ok(Expr::Boolean(checked.is_ok(), bool_ty));
           }

           let (l, slice) = checked.map_err(|e| format!("`reg[...]`: {e}"))?;
           match slice {
               None => {
                   let ty = reg::int_type(self.syms, l.bits, true)?;
                   Ok(Expr::Reg(l, ty))
               }
               Some(s) => {
                    let whole_lane = l.lane.is_some() && raw.range.is_none();

                    let ty = match (&l.lane, raw.range) {
                        (Some(lane), None) => lane.ty.value_type(self.syms)?,
                        _ => reg::int_type(self.syms, slice_value_bits(&l, &s), false)?,
                    };

                    if whole_lane {
                        Ok(Expr::RegSliceGet(l, s, ty))   // already a value
                    } else {
                        Ok(Expr::RegSlice(l, s, ty))      // bit slice: needs `.*` or `= 0b...`
                    }
                }
           }
       }

    fn syscall(&mut self) -> Result<Expr, String> {
        let err = "Expected a hexadecimal syscall number after `syscall` (e.g. `syscall 0x01(...)`)";
        let Some(Hexa(n)) = self.next_token().map(|tk| &tk.kind) else {
            return Err(err.into());
        };
        let n = u64::try_from(*n).map_err(|_| format!("invalid syscall number: {n}"))?;

        let args = self.arguments(&format!("syscall {n:#x}"))?;
        let ret = builtin(self.syms, DEFAULT_SYSCALL_RET)?;
        Ok(Expr::Syscall(n, args, ret))
    }
}

pub fn parse_postfix(
    lhs: Expr,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Expr, String> {
    Parser::new(syms, it).postfix(lhs)
}

pub fn parse_call(
    id: &str,
    ret: CarlaType,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Expr, String> {
    Parser::new(syms, it).call(id, ret)
}

pub fn parse(syms: &mut Symbols, iterator: &mut ContextIterator<'_>) -> Result<Compiled, String> {
    let expr = Parser::new(syms, iterator).expr(0)?;
    Ok(Compiled::from_expr(expr))
}

pub fn is_integer(ty: &CarlaType) -> bool {
    is_int_ty(ty)
}

pub fn range_bounds(
    start: Compiled,
    end: Compiled,
    hint: Option<&CarlaType>,
) -> Result<(Compiled, Compiled, CarlaType), String> {
    let (mut a, mut b) = (start.expr().clone(), end.expr().clone());

    if let Some(h) = hint {
        a = adapt(a, h)?;
        b = adapt(b, h)?;
    }

    match (a.is_const(), b.is_const()) {
        (true, false) => a = adapt(a, &b.ty())?,
        (false, true) => b = adapt(b, &a.ty())?,
        _ => {}
    }

    let (ta, tb) = (a.ty(), b.ty());
    if !is_int_ty(&ta) || !is_int_ty(&tb) {
        return Err(format!(
            "the bounds of a range must be integers, found `{}` and `{}`",
            ta.to_string(),
            tb.to_string()
        ));
    }

    if morgana_of(&ta) != morgana_of(&tb) {
        return Err(format!(
            "mismatched range bounds: `{}` and `{}` (no implicit conversion; use `as`)",
            ta.to_string(),
            tb.to_string()
        ));
    }

    if let Some(h) = hint {
        if morgana_of(h) != morgana_of(&ta) {
            return Err(format!(
                "the range is `{}` but the loop variable is `{}`",
                ta.to_string(),
                h.to_string()
            ));
        }
    }

    let ty = hint.cloned().unwrap_or(ta);
    Ok((Compiled::from_expr(a), Compiled::from_expr(b), ty))
}

pub fn require_bool(c: &Compiled, syms: &Symbols, what: &str) -> Result<(), String> {
    let want = builtin(syms, DEFAULT_BOOL)?.to_string();
    let got = c.expr().ty().to_string();
    if got != want {
        return Err(format!(
            "{what} must be `{want}`, found `{got}` (compare explicitly, e.g. `x != 0`)"
        ));
    }
    Ok(())
}

/// Parses something that can be assigned to: a variable (`x`), `p.*` or `p[i]`.
/// It stops right after the postfix operators, so `raw + i` is NOT a place.
/// `p[i]` / `p.*` come back as `Expr::Deref(pointer, element_type)`.
pub fn parse_place(
    token: &Token,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Expr, String> {
    let mut parser = Parser::new(syms, it);
    let head = parser.prefix(token)?;
    parser.postfix(head)
}

/// Calls run again when a place is read AND written (`p[f()] += 1`), so they're refused there.
fn has_side_effects(e: &Expr) -> bool {
    match e {
        Expr::Call(..) | Expr::CallIndirect(..) | Expr::Syscall(..) => true,
        Expr::Unary(_, x, _) | Expr::Cast(x, _) | Expr::Deref(x, _) | Expr::TempRef(x, _) => {
            has_side_effects(x)
        }
        Expr::Binary(_, l, r, _) => has_side_effects(l) || has_side_effects(r),
        Expr::Pack(items, _) => items.iter().any(has_side_effects),
        _ => false,
    }
}

fn place_name(place: &Expr) -> String {
    match place {
        Expr::Var(n, _) => format!("`{n}`"),
        _ => "this place".into(),
    }
}

/// Parses the right-hand side of an assignment to `place` (see [`parse_place`]).
///
/// * `op == None`      -> `place = <rhs>`
/// * `op == Some(op)`  -> `place <op>= <rhs>`, built as `place <op> <rhs>`, so it gets the same
///   type checks, literal adaptation and pointer arithmetic as any other expression.
///
/// The type of the final value must be exactly the type of the place.
pub fn assignment(
    place: &Expr,
    op: Option<BinOp>,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Compiled, String> {
    let ty = place.ty();
    let rhs = Parser::new(syms, it).expr(0)?;

    let value = match op {
        None => adapt(rhs, &ty)?,
        Some(op) => {
            if has_side_effects(place) {
                return Err(
                    "the left side of a compound assignment has a call in it, which would run \
                     twice; store the call's result in a variable first"
                        .into(),
                );
            }
            make_binary(op, place.clone(), rhs, syms)?
        }
    };

    let nil_in_ptr = matches!(value, Expr::Nil) && is_ptr(&ty);

    let (want, got) = (ty.to_string(), value.ty().to_string());
    if want != got && !nil_in_ptr {
        return Err(format!(
            "can't assign a `{got}` to {}, which is a `{want}` (no implicit conversion; use `as`)",
            place_name(place)
        ));
    }

    Ok(Compiled::from_expr(value))
}

pub fn step(place: &Expr, increment: bool, syms: &Symbols) -> Result<Compiled, String> {
    let ty = place.ty();

    if !is_int_ty(&ty) {
        return Err(format!(
            "can't apply ++/-- to {}: `{}` isn't an integer type",
            place_name(place),
            ty.to_string()
        ));
    }
    if has_side_effects(place) {
        return Err(
            "the place in `++`/`--` has a call in it, which would run twice; store the call's \
             result in a variable first"
                .into(),
        );
    }

    let op = if increment { BinOp::Add } else { BinOp::Sub };
    let value = make_binary(op, place.clone(), Expr::Integer(1, ty), syms)?;
    Ok(Compiled::from_expr(value))
}

pub fn work(
    token: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    let mut parser = Parser::new(syms, iterator);

    let head = parser.prefix(token)?;
    let expr = parser.continue_expr(head, 0)?;
    let compiled = Compiled::from_expr(expr);

    let Some(next) = parser.it.next() else {
        return Ok(Expression(compiled, true));
    };

    match next {
        Common(finish) if matches!(finish.kind, Comma | Semi) => Ok(Expression(compiled, false)),
        Common(tok) => Err(format!(
            "Expected ; or , after an expression, found {:?}",
            tok.kind
        )),
        _ => Err("Expected ; or , after an expression, found a block `(...)`/`[...]`/`{...}`".into()),
    }
}
