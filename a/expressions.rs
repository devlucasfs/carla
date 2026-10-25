use std::sync::atomic::{AtomicU64, Ordering};

use crate::{
    compiler::{
        abi,
        iterator::ContextIterator,
        nesting::Context::{self, Block, Common},
        node::Node::{self, Expression},
        patterns::types::{self, Base, CarlaType},
        scanner::{
            BitOp::AND,
            Keywords::{As, Syscall},
            MathOp::{self, Minus},
            PBB::Paren,
            Token,
            TokenKind::{
                self, Bang, BitOperator, Bool, Comma, Dot, Float, Hexa, Identifier, Integer,
                Keyword, MathOperator, Nil, Octal, Semi, String as Str,
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
const DEFAULT_STR_BASE: &str = "byte";
/// Syscall result type: the machine word returned in rax.
/// Should be the 64-bit type registered in Symbols.
const DEFAULT_SYSCALL_RET: &str = "int";

/// Binding powers (left, right) of the Pratt parser, from loosest to tightest.
mod bp {
    pub const BIT_AND: (u8, u8) = (3, 4);
    pub const ADDITIVE: (u8, u8) = (5, 6);
    pub const MULTIPLICATIVE: (u8, u8) = (7, 8);
    pub const CAST: u8 = 9;
    pub const PREFIX: u8 = 11;
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
}

impl BinOp {
    fn from_token(kind: &TokenKind) -> Option<(Self, u8, u8)> {
        let (op, (left, right)) = match kind {
            BitOperator(AND) => (Self::BitAnd, bp::BIT_AND),
            MathOperator(MathOp::Plus) => (Self::Add, bp::ADDITIVE),
            MathOperator(MathOp::Minus) => (Self::Sub, bp::ADDITIVE),
            MathOperator(MathOp::Star) => (Self::Mul, bp::MULTIPLICATIVE),
            MathOperator(MathOp::Slash) => (Self::Div, bp::MULTIPLICATIVE),
            MathOperator(MathOp::Mod) => (Self::Rem, bp::MULTIPLICATIVE),
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
    Syscall(u64, Vec<Expr>, CarlaType),
    Unary(UnOp, Box<Expr>, CarlaType),
    Binary(BinOp, Box<Expr>, Box<Expr>, CarlaType),
    Cast(Box<Expr>, CarlaType),
    /// `ptr.*`: the value a pointer points to (the type is the pointee type).
    Deref(Box<Expr>, CarlaType),
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
            | Expr::Syscall(_, _, t)
            | Expr::Unary(_, _, t)
            | Expr::Binary(_, _, _, t)
            | Expr::Cast(_, t)
            | Expr::Deref(_, t) => t.clone(),
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

    fn from_expr(expr: Expr) -> Self {
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
    Syscall(u64, Vec<Operand>),
    Cast(Operand, CarlaType),
    Deref(Operand),
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
        Expr::Binary(op, l, r, t) => {
            let a = lower(l, out);
            let b = lower(r, out);
            emit(out, t, StepOp::Binary(*op, a, b))
        }
        Expr::Call(n, args, t) => {
            let a = lower_all(args, out);
            emit(out, t, StepOp::Call(n.clone(), a))
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

fn is_int_ty(t: &CarlaType) -> bool {
    !is_ptr(t) && morgana_of(t).map_or(false, |m| is_int(&m))
}

fn is_word(t: &CarlaType) -> bool {
    !is_ptr(t)
        && matches!(
            morgana_of(t),
            Some(MorganaType::Integer(64)) | Some(MorganaType::UInteger(64))
        )
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

fn type_binary(op: BinOp, l: &CarlaType, r: &CarlaType) -> Result<CarlaType, String> {
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

    Ok(l.clone())
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
        (UnOp::Not, Expr::Integer(i, _)) => Some(Expr::Integer((*i == 0) as i128, ty.clone())),
        _ => None,
    };
    Ok(folded.unwrap_or_else(|| Expr::Unary(op, Box::new(x), ty)))
}

fn make_binary(op: BinOp, l: Expr, r: Expr) -> Result<Expr, String> {
    let (l, r) = match (l.is_const(), r.is_const()) {
        (true, false) => (adapt(l, &r.ty())?, r),
        (false, true) => {
            let t = l.ty();
            (l, adapt(r, &t)?)
        }
        _ => (l, r),
    };

    let ty = type_binary(op, &l.ty(), &r.ty())?;
    if let Some(folded) = fold_binary(op, &l, &r, &ty)? {
        return Ok(folded);
    }
    Ok(Expr::Binary(op, Box::new(l), Box::new(r), ty))
}

fn fold_binary(op: BinOp, l: &Expr, r: &Expr, ty: &CarlaType) -> Result<Option<Expr>, String> {
    use BinOp::*;

    if let (Expr::Boolean(a, _), Expr::Boolean(b, _)) = (l, r) {
        return Ok((op == BitAnd).then(|| Expr::Boolean(a & b, ty.clone())));
    }

    let (Expr::Integer(a, _), Expr::Integer(b, _)) = (l, r) else {
        return Ok(None);
    };

    let v = match op {
        Div | Rem if *b == 0 => return Err("compile-time division by zero".into()),
        Add => a.checked_add(*b),
        Sub => a.checked_sub(*b),
        Mul => a.checked_mul(*b),
        Div => a.checked_div(*b),
        Rem => a.checked_rem(*b),
        BitAnd => Some(a & b),
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

/// The pointer type behind `ty`, seeing through aliases: `string` (`using string = byte*`)
/// becomes `byte*`. The alias stays the type of the expression; this view only gives
/// pointer operations the real pointer underneath.
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

    // `nil` and `|_|*` have no pointee type: there is no value to read
    if pointee.levels == 0 && pointee.base.is_none() && pointee.tuple.is_empty() {
        return Err("cannot dereference an untyped pointer (cast it first, e.g. `as T*`)".into());
    }

    if pointee.levels == 0 && !pointee.dim.is_empty() {
        return Err("dereferencing a pointer to an array isn't supported yet".into());
    }

    Ok(Expr::Deref(Box::new(x), pointee))
}

fn self_ref(receiver: Expr) -> Result<Expr, String> {
    match receiver {
        Expr::Var(name, ty) => Ok(Expr::Pointer(name, ty.pointer_to())),
        other => Err(format!(
            "cannot take a reference to a temporary value of type `{}` to use as `self`; \
             store it in a variable first",
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

pub fn starts_with(kind: &TokenKind) -> bool {
    matches!(
        kind,
        Identifier(_)
            | Keyword(Syscall)
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

    fn peek_binary_op(&mut self) -> Option<(BinOp, u8, u8)> {
        match self.it.peek() {
            Some(Common(tk)) => BinOp::from_token(&tk.kind),
            _ => None,
        }
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
            lhs = make_binary(op, lhs, rhs)?;
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

    /// Postfix `.`:
    ///   `value.*`              reads the value behind a pointer
    ///   `value.function(args)` calls an extended function, passing `&value` as `self`
    fn postfix(&mut self, mut lhs: Expr) -> Result<Expr, String> {
        while self.at(&Dot) {
            self.it.next();

            if self.at(&MathOperator(MathOp::Star)) {
                self.it.next();
                lhs = make_deref(lhs)?;
                continue;
            }

            let Some(member) = self.next_identifier() else {
                return Err("Expected a function name or `*` after '.'".into());
            };
            let owner = lhs.ty();
            lhs = self.extended_call(&owner, Some(lhs), member)?;
        }
        Ok(lhs)
    }

    fn prefix(&mut self, token: &Token) -> Result<Expr, String> {
        match &token.kind {
            Keyword(Syscall) => self.syscall(),

            MathOperator(Minus) => make_unary(UnOp::Neg, self.expr(bp::PREFIX)?),
            Bang => make_unary(UnOp::Not, self.expr(bp::PREFIX)?),
            BitOperator(AND) => self.address_of(),

            Integer(i) | Hexa(i) | Octal(i) => int_literal(*i, self.syms),
            Bool(b) => Ok(Expr::Boolean(*b, builtin(self.syms, DEFAULT_BOOL)?)),
            Str(s) => Ok(Expr::String(
                s.clone(),
                builtin(self.syms, DEFAULT_STR_BASE)?.pointer_to(),
            )),
            Nil => Ok(Expr::Nil),
            Float(_) => Err("Morgana doesn't have a float type yet".into()),

            Identifier(id) => self.identifier(token, id),

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

    fn identifier(&mut self, token: &Token, id: &str) -> Result<Expr, String> {
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

    /// `Type.function(args)`: a function extended onto a type, called without a receiver.
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

        self.extended_call(&owner, None, member)
    }

    fn extended_call(
        &mut self,
        owner: &CarlaType,
        receiver: Option<Expr>,
        member: &str,
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
        let mut args = self.arguments(&full)?;

        if let Some(receiver) = receiver {
            args.insert(0, self_ref(receiver)?);
        }

        Ok(Expr::Call(full, args, ret))
    }

    fn call(&mut self, id: &str, ret: CarlaType) -> Result<Expr, String> {
        let args = self.arguments(id)?;
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
        _ => Err("Expected ; or , and also maybe EOF, after an expression".into()),
    }
}
