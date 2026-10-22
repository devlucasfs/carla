use std::sync::atomic::{AtomicU64, Ordering};

use crate::{
    compiler::{
        abi,
        iterator::ContextIterator,
        nesting::Context::{Block, Common},
        node::Node::{self, Expression},
        patterns::types::{self, Base, CarlaType},
        scanner::{
            BitOp::AND,
            Keywords::{As, Syscall},
            MathOp::{self, Minus},
            PBB::Paren,
            Token,
            TokenKind::{
                self, Bang, Bool, Comma, Dot, Float, Hexa, Identifier, Integer, Keyword,
                MathOperator, Nil, Octal, Semi, String as Str,
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
/// Tipo de retorno de uma syscall (palavra de máquina em rax).
/// Troque pelo tipo de 64 bits registrado em Symbols.
const DEFAULT_SYSCALL_RET: &str = "int";

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
    /// `ptr.*`: reads the value a pointer points to (the CarlaType is the pointee type)
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
            let a = args.iter().map(|x| lower(x, out)).collect();
            emit(out, t, StepOp::Call(n.clone(), a))
        }
        Expr::Syscall(n, args, t) => {
            let a = args.iter().map(|x| lower(x, out)).collect();
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
        lit => Operand::Const(lit.clone()),
    }
}

fn morgana_of(ty: &CarlaType) -> Option<MorganaType> {
    if !ty.dim.is_empty() {
        return None;
    }
    if ty.levels == 0 && ty.tuple.is_empty() && ty.base.is_none() {
        return None;
    }
    Some(ty.to_morgana())
}

fn is_int(m: &MorganaType) -> bool {
    matches!(m, MorganaType::Integer(_) | MorganaType::UInteger(_))
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
            "o valor {v} não cabe em `{}` (overflow em tempo de compilação)",
            ty.to_string()
        )),
        _ => Ok(()),
    }
}

fn unsupported(op: impl std::fmt::Debug, tys: &[&CarlaType]) -> String {
    let names: Vec<String> = tys.iter().map(|t| format!("`{}`", t.to_string())).collect();
    format!(
        "operador {op:?} não se aplica a {} (overload de operador ainda não suportado)",
        names.join(" e ")
    )
}

fn type_unary(op: UnOp, t: &CarlaType) -> Result<CarlaType, String> {
    match (op, morgana_of(t)) {
        (UnOp::Neg, Some(MorganaType::Integer(_))) => Ok(t.clone()),
        (UnOp::Neg, Some(MorganaType::UInteger(_))) => {
            Err(format!("negação de `{}`: tipo sem sinal", t.to_string()))
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
            "tipos diferentes em {op:?}: `{}` e `{}` (sem conversão implícita; use `as`)",
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
            let v = i.checked_neg().ok_or("overflow em tempo de compilação")?;
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
    if let Some(v) = fold_binary(op, &l, &r, &ty)? {
        return Ok(v);
    }
    Ok(Expr::Binary(op, Box::new(l), Box::new(r), ty))
}

fn fold_binary(op: BinOp, l: &Expr, r: &Expr, ty: &CarlaType) -> Result<Option<Expr>, String> {
    use BinOp::*;

    if let (Expr::Boolean(a, _), Expr::Boolean(b, _)) = (l, r) {
        if op == BitAnd {
            return Ok(Some(Expr::Boolean(a & b, ty.clone())));
        }
        return Ok(None);
    }

    let (Expr::Integer(a, _), Expr::Integer(b, _)) = (l, r) else {
        return Ok(None);
    };

    let v = match op {
        Div | Rem if *b == 0 => return Err("divisão por zero em tempo de compilação".into()),
        Add => a.checked_add(*b),
        Sub => a.checked_sub(*b),
        Mul => a.checked_mul(*b),
        Div => a.checked_div(*b),
        Rem => a.checked_rem(*b),
        BitAnd => Some(a & b),
    }
    .ok_or("overflow em tempo de compilação")?;

    check_fits(v, ty)?;
    Ok(Some(Expr::Integer(v, ty.clone())))
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

fn make_cast(x: Expr, to: CarlaType) -> Result<Expr, String> {
    let from = x.ty();
    let (fs, ts) = (from.to_string(), to.to_string());

    if fs == ts {
        return Ok(x);
    }

    let (fp, tp) = (is_ptr(&from), is_ptr(&to));

    let compatible = (is_int_ty(&from) && is_int_ty(&to)) // integer <-> integer
        || (fp && tp)                                      // pointer <-> pointer
        || (fp && is_word(&to))                            // pointer -> 64-bit integer
        || (is_word(&from) && tp); // 64-bit integer -> pointer

    if !compatible {
        return Err(format!(
            "não é possível converter `{fs}` para `{ts}` \
             (só inteiro <-> inteiro, ponteiro <-> ponteiro e ponteiro <-> inteiro de 64 bits)"
        ));
    }

    if is_int_ty(&to) {
        let folded = match &x {
            Expr::Integer(v, _) => Some(*v),
            Expr::Boolean(b, _) => Some(*b as i128),
            _ => None,
        };
        if let Some(v) = folded {
            check_fits(v, &to)?;
            return Ok(Expr::Integer(v, to));
        }
    }

    Ok(Expr::Cast(Box::new(x), to))
}

/// `ptr.*`: the result has the pointee type (one `*` less).
fn make_deref(x: Expr) -> Result<Expr, String> {
    let ty = x.ty();

    if ty.levels == 0 {
        return Err(if is_ptr(&ty) {
            format!(
                "cannot dereference `{}`: it is an untyped pointer (cast it first, e.g. `as T*`)",
                ty.to_string()
            )
        } else {
            format!("cannot dereference `{}`: it is not a pointer", ty.to_string())
        });
    }

    let mut pointee = ty.clone();
    pointee.levels -= 1;

    // `nil` and other untyped pointers (`|_|*`): there is no value to read
    if pointee.levels == 0 && pointee.base.is_none() && pointee.tuple.is_empty() {
        return Err("cannot dereference an untyped pointer (cast it first, e.g. `as T*`)".into());
    }

    if pointee.levels == 0 && !pointee.dim.is_empty() {
        return Err("dereferencing a pointer to an array isn't supported yet".into());
    }

    Ok(Expr::Deref(Box::new(x), pointee))
}

pub fn starts_with(kind: &TokenKind) -> bool {
    matches!(
        kind,
        Identifier(_)
            | Keyword(Syscall)
            | Bool(_)
            | TokenKind::BitOperator(AND)
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

pub fn can_make_ptr(sym: &Symbol) -> bool {
    matches!(sym, Variable(_, _, _) | ExternFunction(_, _))
}

fn builtin(syms: &Symbols, id: &str) -> Result<CarlaType, String> {
    CarlaType::named(id, syms).ok_or_else(|| format!("tipo '{id}' não está registrado em Symbols"))
}

fn symbol_ty(id: &str, sym: &Symbol) -> Result<CarlaType, String> {
    match sym {
        Variable(Some(t), _, _) => Ok((**t).clone()),
        Variable(None, _, _) => Err(format!("a variável '{id}' ainda não tem tipo definido")),
        ExternFunction(ret, _) | UnfoldFunction(ret, _) => Ok((**ret).clone()),
        _ => Err(format!("'{id}' não é um valor")),
    }
}

fn pointer_ty(id: &str, sym: &Symbol) -> Result<CarlaType, String> {
    match sym {
        Variable(Some(t), _, false) => Ok(t.pointer_to()),
        Variable(None, _, false) => Err(format!("a variável '{id}' ainda não tem tipo definido")),
        _ => Ok(CarlaType::untyped_ptr()),
    }
}

fn int_literal(v: i128, syms: &Symbols) -> Result<Expr, String> {
    let ty = builtin(syms, DEFAULT_INT)?;
    check_fits(v, &ty)?;
    Ok(Expr::Integer(v, ty))
}

fn binop_of(k: &TokenKind) -> Option<(BinOp, u8, u8)> {
    Some(match k {
        TokenKind::BitOperator(AND) => (BinOp::BitAnd, 3, 4),
        MathOperator(MathOp::Plus) => (BinOp::Add, 5, 6),
        MathOperator(MathOp::Minus) => (BinOp::Sub, 5, 6),
        MathOperator(MathOp::Star) => (BinOp::Mul, 7, 8),
        MathOperator(MathOp::Slash) => (BinOp::Div, 7, 8),
        MathOperator(MathOp::Mod) => (BinOp::Rem, 7, 8),
        _ => return None,
    })
}

const AS_BP: u8 = 9;
const PREFIX_BP: u8 = 11;

fn parse_expr(
    min_bp: u8,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Expr, String> {
    let lhs = match it.next() {
        None => return Err("expressão incompleta".into()),
        Some(Block(pbb, inner)) if *pbb == Paren => {
            let mut sub = ContextIterator::new(inner.as_slice(), 0);
            let v = parse_expr(0, syms, &mut sub)?;
            if sub.remaining() > 0 {
                return Err("tokens sobrando dentro dos parênteses".into());
            }
            v
        }
        Some(Common(tk)) => parse_prefix(tk, syms, it)?,
        Some(_) => return Err("bloco inesperado no início da expressão".into()),
    };
    let lhs = parse_postfix(lhs, syms, it)?;
    climb(lhs, min_bp, syms, it)
}

fn climb(
    mut lhs: Expr,
    min_bp: u8,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Expr, String> {
    loop {
        let Some(Common(tk)) = it.peek() else { break };

        if tk.kind == Keyword(As) {
            if AS_BP < min_bp {
                break;
            }
            it.next();

            let err = "Expected a type after `as`";
            let Some(Common(first)) = it.next() else {
                return Err(err.into());
            };
            let target = types::parse_path(first, syms, it)?;
            lhs = make_cast(lhs, target)?;
            continue;
        }

        let Some((op, lbp, rbp)) = binop_of(&tk.kind) else {
            break;
        };
        if lbp < min_bp {
            break;
        }
        it.next();
        let rhs = parse_expr(rbp, syms, it)?;
        lhs = make_binary(op, lhs, rhs)?;
    }
    Ok(lhs)
}

/// Postfix `.`:
///   `value.*`              reads the value behind a pointer
///   `value.function(args)` calls an extended function, passing `&value` as `self`
pub fn parse_postfix(
    mut lhs: Expr,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Expr, String> {
    while let Some(Common(tk)) = it.peek() {
        if tk.kind != Dot {
            break;
        }
        it.next();

        // `.*`: dereference
        if let Some(Common(star)) = it.peek() {
            if star.kind == MathOperator(MathOp::Star) {
                it.next();
                lhs = make_deref(lhs)?;
                continue;
            }
        }

        let err = "Expected a function name or `*` after '.'";
        let Some(Common(member)) = it.next() else {
            return Err(err.into());
        };
        let Identifier(member_id) = &member.kind else {
            return Err(err.into());
        };

        let owner = lhs.ty();
        lhs = extended_call(&owner, Some(lhs), member_id, syms, it)?;
    }
    Ok(lhs)
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

fn extended_call(
    owner: &CarlaType,
    receiver: Option<Expr>,
    member: &str,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Expr, String> {
    let full = abi::extension_symbol(owner, member);

    let Some(sym) = syms.lookup(&full) else {
        return Err(format!(
            "`{}` has no extended function `{member}` ({full})",
            owner.to_string()
        ));
    };

    if !matches!(
        sym,
        Variable(_, _, true) | ExternFunction(_, _) | UnfoldFunction(_, _)
    ) {
        return Err(format!("`{full}` isn't a function"));
    }

    let ret = symbol_ty(&full, sym)?;
    let Expr::Call(name, mut args, ty) = parse_call(&full, ret, syms, it)? else {
        unreachable!()
    };

    if let Some(r) = receiver {
        args.insert(0, self_ref(r)?);
    }

    Ok(Expr::Call(name, args, ty))
}

fn parse_prefix(
    token: &Token,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Expr, String> {
    match &token.kind {
        Keyword(Syscall) => parse_syscall(syms, it),

        MathOperator(Minus) => make_unary(UnOp::Neg, parse_expr(PREFIX_BP, syms, it)?),
        Bang => make_unary(UnOp::Not, parse_expr(PREFIX_BP, syms, it)?),

        TokenKind::BitOperator(AND) => {
            let err = "Expected an identifier after a & (pointer operator)";
            let Some(Common(tk)) = it.next() else {
                return Err(err.into());
            };
            let Identifier(identifier) = &tk.kind else {
                return Err(err.into());
            };
            let Some(sym) = syms.lookup(identifier) else {
                return Err(format!("{identifier} was not found"));
            };
            if !can_make_ptr(sym) {
                return Err(format!("you can't make a pointer of {identifier}"));
            }
            let ty = pointer_ty(identifier, sym)?;
            Ok(Expr::Pointer(identifier.clone(), ty))
        }

        Integer(i) | Hexa(i) | Octal(i) => int_literal(*i, syms),
        Bool(b) => Ok(Expr::Boolean(*b, builtin(syms, DEFAULT_BOOL)?)),
        Str(s) => Ok(Expr::String(
            s.clone(),
            builtin(syms, DEFAULT_STR_BASE)?.pointer_to(),
        )),
        Nil => Ok(Expr::Nil),
        Float(_) => Err("a Morgana ainda não tem tipo float".into()),

        Identifier(id) => {
            let Some(sym) = syms.lookup(id) else {
                return Err(format!("simbolo n existente : '{id}'"));
            };

            let is_type = matches!(sym, BType(_) | TypeAlias(_));
            if is_type {
                let owner = types::parse_path(token, syms, it)?;

                let err = format!(
                    "`{}` is a type, not a value; use `{}.function(...)`",
                    owner.to_string(),
                    owner.to_string()
                );
                let Some(Common(dot)) = it.next() else {
                    return Err(err);
                };
                if dot.kind != Dot {
                    return Err(err);
                }
                let Some(Common(member)) = it.next() else {
                    return Err(err);
                };
                let Identifier(member_id) = &member.kind else {
                    return Err(err);
                };

                return extended_call(&owner, None, member_id, syms, it);
            }

            let is_call = match sym {
                Variable(_, _, func) => *func,
                ExternFunction(_, _) => true,
                UnfoldFunction(_, _) => true,
                _ => return Err(format!("{id} isn't a valid identifier")),
            };

            let ty = symbol_ty(id, sym)?;

            if is_call {
                parse_call(id, ty, syms, it)
            } else {
                Ok(Expr::Var(id.clone(), ty))
            }
        }

        _ => Err(format!(
            "token não suportado em expressão: {:?}",
            token.kind
        )),
    }
}

fn parse_args(
    id: &str,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Vec<Expr>, String> {
    let err = format!("Invalid {id} call");
    let Some(Block(pbb, ctx)) = it.next() else {
        return Err(err);
    };
    if *pbb != Paren {
        return Err(err);
    }

    let mut args = vec![];
    let mut sub = ContextIterator::new(ctx.as_slice(), 0);
    while sub.remaining() > 0 {
        args.push(parse_expr(0, syms, &mut sub)?);
        match sub.next() {
            None => break,
            Some(Common(t)) if matches!(t.kind, Comma) => {}
            _ => return Err(err),
        }
    }
    Ok(args)
}

pub fn parse_call(
    id: &str,
    ret: CarlaType,
    syms: &mut Symbols,
    it: &mut ContextIterator<'_>,
) -> Result<Expr, String> {
    let args = parse_args(id, syms, it)?;
    Ok(Expr::Call(id.to_string(), args, ret))
}

fn parse_syscall(syms: &mut Symbols, it: &mut ContextIterator<'_>) -> Result<Expr, String> {
    let err = "Expected a hexadecimal syscall number after `syscall` (e.g. `syscall 0x01(...)`)";
    let Some(Common(tk)) = it.next() else {
        return Err(err.into());
    };
    let Hexa(n) = &tk.kind else {
        return Err(err.into());
    };
    let n = u64::try_from(*n).map_err(|_| format!("invalid syscall number: {n}"))?;

    let args = parse_args(&format!("syscall {n:#x}"), syms, it)?;
    let ret = builtin(syms, DEFAULT_SYSCALL_RET)?;
    Ok(Expr::Syscall(n, args, ret))
}

pub fn work(
    token: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    let lhs = parse_prefix(token, syms, iterator)?;
    let lhs = parse_postfix(lhs, syms, iterator)?;
    let expr = climb(lhs, 0, syms, iterator)?;

    let compiled = if expr.is_const() {
        Compiled::Const(expr)
    } else {
        Compiled::Lazy(ExprIterator::new(expr))
    };

    let Some(next) = iterator.next() else {
        return Ok(Expression(compiled, true));
    };

    let err = "Expected ; or , and also maybe EOF, after an expression";
    let Common(finish) = next else {
        return Err(err.into());
    };
    match finish.kind {
        Comma | Semi => Ok(Expression(compiled, false)),
        _ => Err(err.into()),
    }
}
