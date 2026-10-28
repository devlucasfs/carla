use crate::{
    compiler::{
        node::Node::{
            self, Declaration, Declare, Expression, Extension, Function, Nop, STMTDefer, STMTReturn, STMTReturnTuple, Unfoldable,
        }, patterns::{
            assign::{Assign, Target},
            binding::Binding,
            conditional::Conditional,
            expressions::{BinOp, Compiled, Expr, Operand, Step, StepOp},
            r#extern::Extern,
            for_loop::ForLoop,
            incdec::IncDec,
            lambda::LambdaRunner,
            types::CarlaType,
            unfold::Unfold,
            while_loop::WhileLoop,
        }, symbols::{
            Symbol::{ExternFunction, UnfoldFunction, Variable},
            Symbols,
        }, visitor,
    }, morgana::{
        self, MorganaContext, MorganaExpr, MorganaGenerator,
        MorganaType::{self, Void},
    }, utils::{compiler_outputs::CompilerOutputs, uuid},
};

#[derive(Debug, Clone, Default)]
pub struct Codegen {
    pub nodes: Vec<Node>,
    pub code: String,
    pub extending: Option<CarlaType>,
    pub loops: Vec<(String, String)>,
    pub value_to: Option<String>,
    pub ret: Option<CarlaType>,
}

fn fresh_name(prefix: &str) -> String {
    [prefix, &uuid::gen_hex(16 * 3)].concat()
}

fn binop_name(op: BinOp) -> &'static str {
    match op {
        BinOp::Add => "add",
        BinOp::Sub => "sub",
        BinOp::Mul => "mul",
        BinOp::Div => "div",
        BinOp::Rem => "rem",
        BinOp::BitAnd => "and",
        BinOp::Eq => "eq",
        BinOp::Ne => "ne",
        BinOp::Lt => "lt",
        BinOp::Le => "le",
        BinOp::Gt => "gt",
        BinOp::Ge => "ge",
        BinOp::LogAnd => "and",
        BinOp::LogOr => "or",
    }
}

fn is_signed(t: &MorganaType) -> bool {
    !matches!(t, MorganaType::UInteger(_) | MorganaType::Ptr)
}

fn is_literal(v: &str) -> bool {
    v.parse::<i128>().is_ok()
}

fn ir_declare(name: &str, ty: &MorganaType, value: &str) -> String {
    match ty {
        MorganaType::Tuple(_) => morgana::ir_move(name, value),
        _ => format!(
            "{}\n{}",
            morgana::ir_alloc(name, ty),
            morgana::ir_store(name, value)
        ),
    }
}

fn ensure_symbol(sym: &mut Symbols, id: &str, ty: &CarlaType) {
    if sym.lookup(id).is_none() {
        sym.push(id, Variable(Some(Box::new(ty.clone())), true, false));
    }
}

fn encode_literal(s: &str) -> String {
    let bytes: Vec<u8> = if s.chars().all(|c| (c as u32) <= 0xFF) {
        s.chars().map(|c| c as u8).collect()
    } else {
        s.as_bytes().to_vec()
    };

    let mut out = String::with_capacity(1 + bytes.len() * 2);
    out.push('h');
    for b in bytes {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

impl<'a> MorganaExpr<'a> {
    fn fresh(&mut self) -> String {
        ["t", &uuid::gen_hex(16 * 3)].concat()
    }

    fn fresh_label(&mut self) -> String {
        fresh_name("L")
    }

    fn line(&mut self, s: String) {
        self.out.push_str(&s);
        self.out.push('\n');
    }

    fn as_name(&mut self, v: &str, ty: &CarlaType) -> String {
        if !is_literal(v) {
            return v.to_string();
        }

        let t = fresh_name("c");
        self.line(ir_declare(&t, &ty.to_morgana(), v));
        t
    }

    fn step(&mut self, s: &Step) -> Result<(), String> {
        let (val, ty) = match &s.op {
            StepOp::Call(name, args) => {
                let mut parts = vec![];
                for a in args {
                    let (v, _) = self.value(a)?;
                    parts.push(v);
                }

                let ty = match self.syms.lookup(&name).unwrap() {
                    Variable(Some(ty), _, _) => ty.as_ref().clone(),
                    ExternFunction(ty, _) | UnfoldFunction(ty, _) => ty.as_ref().clone(),
                    _ => todo!(),
                };

                let t = self.fresh();
                if ty.to_morgana() == MorganaType::Void {
                    self.line(format!("call {name}({})", parts.join(", ")));
                } else {
                    self.line(format!("{t} = call {name}({})", parts.join(", ")));
                }

                (t, s.ty.clone())
            }

            StepOp::AddrOf(id) => {
                let ty = match self.syms.lookup(id).unwrap() {
                    Variable(Some(ty), _, _) => ty.as_ref().clone(),
                    _ => todo!(),
                };

                let t = self.fresh();
                self.line(format!("{t} = ptr_to {id}"));

                (t, ty.pointer_to())
            }

            StepOp::Syscall(number, args) => {
                let mut parts = vec![number.to_string()];
                for a in args {
                    let (v, _) = self.value(a)?;
                    parts.push(v);
                }

                let t = self.fresh();
                self.line(format!("{t} = syscall {}", parts.join(", ")));

                (t, s.ty.clone())
            }

            StepOp::Cast(op, from) => {
                let (v, _) = self.value(op)?;

                if from.to_morgana() == s.ty.to_morgana() || is_literal(&v) {
                    (v, s.ty.clone())
                } else {
                    let t = self.fresh();
                    self.line(format!("{t} = cast {} {v}", s.ty.to_morgana().to_string()));
                    (t, s.ty.clone())
                }
            }

            StepOp::Deref(op) => {
                let (p, _) = self.value(op)?;
                let t = self.fresh();
                self.line(format!("{t} = deref {} {p}", s.ty.to_morgana().to_string()));

                (t, s.ty.clone())
            }

            StepOp::Binary(op, a, b) => {
                let (x, tx) = self.value(a)?;
                let (y, _) = self.value(b)?;

                let t = self.fresh();
                self.line(morgana::ir_binary(
                    &t,
                    binop_name(*op),
                    is_signed(&tx.to_morgana()),
                    &x,
                    &y,
                ));

                (t, s.ty.clone())
            }

            StepOp::Logical(op, lhs, rhs_steps, rhs) => {
                let (l, tl) = self.value(lhs)?;
                let cond = self.as_name(&l, &tl);

                let res = self.fresh();
                let skip = self.fresh_label();

                self.line(morgana::ir_alloc(&res, &s.ty.to_morgana()));
                self.line(morgana::ir_store(&res, &l));
                self.line(if *op == BinOp::LogAnd {
                    morgana::ir_jz(&cond, &skip)
                } else {
                    morgana::ir_jnz(&cond, &skip)
                });

                for st in rhs_steps {
                    self.step(st)?;
                }

                let (r, _) = self.value(rhs)?;
                self.line(morgana::ir_store(&res, &r));
                self.line(morgana::ir_label(&skip));

                (res, s.ty.clone())
            }

            _ => todo!(),
        };

        self.vals.insert(s.id, (val, ty));
        Ok(())
    }

    fn value(&mut self, op: &Operand) -> Result<(String, CarlaType), String> {
        match op {
            Operand::Const(e) => Ok(match e {
                Expr::Integer(i, ty) => (i.to_string(), ty.clone()),
                // Expr::Float(f, ty) => (format!("0x{:016X}", f.to_bits()), ty),
                Expr::Boolean(b, ty) => ((if *b { "255" } else { "0" }).into(), ty.clone()),
                Expr::Nil => ("0".into(), CarlaType::from_basic("generic_ptr")),
                Expr::String(s, ty) => {
                    let t = self.fresh();
                    self.line(format!("{t} = str {}", encode_literal(s)));
                    (t, ty.clone())
                }
                _ => return Err("constante não suportada no emissor (ex.: string)".into()),
            }),

            Operand::Temp(id) => self
                .vals
                .get(id)
                .cloned()
                .ok_or_else(|| "temporário usado antes de ser definido".to_string()),

            Operand::Var(name) => {
                let ty = match self.syms.lookup(&name).unwrap() {
                    Variable(Some(ty), _, _) => ty.as_ref().clone(),
                    _ => todo!(),
                };

                let t = self.fresh();
                self.line(format!("{t} = load {name}"));
                Ok((t, ty.clone()))
            }
        }
    }
}

impl<'a> MorganaGenerator<Compiled> for MorganaExpr<'a> {
    fn to_morgana(&mut self, data: &Compiled) -> (String, CarlaType) {
        match data {
            Compiled::Const(expr) => self.value(&Operand::Const(expr.clone())).unwrap(),

            Compiled::Lazy(iterator) => {
                let mut it = iterator.clone();
                let result = it.result().clone();

                for step in &mut it {
                    _ = self.step(&step);
                }

                self.value(&result).unwrap()
            }
        }
    }
}

impl Codegen {
    pub fn new(nodes: Vec<Node>) -> Self {
        Codegen {
            nodes,
            ..Default::default()
        }
    }

    pub fn append<T: ToString>(&mut self, data: T) {
        self.code.push_str(&data.to_string());
    }

    pub fn generate(&mut self, sym: &mut Symbols, ret: Option<CarlaType>) -> String {
        self.ret = ret.clone();

        let mut defers = vec![];

        let make_defers = |r: &mut Codegen, sym: &mut Symbols, defers: &mut Vec<Compiled>| {
            let mut d = defers.clone();
            while let Some(defer) = d.pop() {
                let mut s = sym.clone();
                let mut morg = MorganaExpr::new(MorganaContext::Declaration, &mut s);
                _ = morg.to_morgana(&defer);
                morgana::write(r, &morg.out);
            }
        };

        while self.nodes.len() > 0 {
            match (self.nodes.remove(0), (self.nodes.clone()).first()) {
                (Nop, _) => {}
                (STMTDefer(expr), _) => defers.push(expr),
                (Declaration(decl), Some(Function(func))) => {
                    if decl._type.is_none() {
                        CompilerOutputs::fatal("You can't define a function without a return type");
                    }

                    self.nodes.remove(0);

                    let self_ty: Option<CarlaType> = if func.extension {
                        let Some(owner) = &self.extending else {
                            CompilerOutputs::fatal(
                                "`self` is only allowed in functions inside an extend block",
                            );
                            unreachable!();
                        };
                        Some(owner.pointer_to())
                    } else {
                        None
                    };

                    let mut params: Vec<(String, CarlaType)> = vec![];

                    if let Some(t) = &self_ty {
                        params.push(("self".to_string(), t.clone()));
                    }

                    for arg in &func.args {
                        for id in &arg.ids {
                            if params.iter().any(|(name, _)| name == id) {
                                CompilerOutputs::fatal(format!("Duplicated parameter '{id}'"));
                            }
                            params.push((id.clone(), arg.ty.as_ref().clone()));
                        }
                    }

                    let args: Vec<_> = params
                        .iter()
                        .map(|(name, ty)| (name.clone(), ty.to_morgana()))
                        .collect();

                    let ret_ty = decl._type.clone().unwrap().as_ref().clone();

                    sym.push_scope();

                    for (name, ty) in &params {
                        sym.push(name, Variable(Some(Box::new(ty.clone())), true, false));
                    }

                    let runner = match &func.run {
                        LambdaRunner::Scope(ctx) => visitor::work(&ctx, sym),
                        LambdaRunner::Expr(expr) => vec![Expression(expr.clone(), true)],
                        _ => todo!(),
                    };

                    let mut cgen = Codegen::new(runner);
                    let body = cgen.generate(sym, Some(ret_ty));

                    if matches!(&func.run, LambdaRunner::Scope(_)) {
                        sym.pop_scope();
                    }
                    sym.pop_scope();

                    let ctype = decl._type.unwrap();
                    morgana::function(self, ctype.to_morgana(), decl.identifier, args, body);
                }

                (Declaration(decl), Some(Expression(expr, _))) => {
                    let mut s = sym.clone();
                    let mut morg = MorganaExpr::new(MorganaContext::Declaration, &mut s);
                    let (result, ty) = morg.to_morgana(&expr.clone());
                    self.nodes.remove(0);

                    let irty = ty.to_morgana();

                    if let Some(tty) = decl._type {
                        let (x, y) = (tty.as_ref().to_string(), ty.to_string());
                        if x != y {
                            CompilerOutputs::fatal(format!(
                                "You defined {} as a {x} but was found {y}",
                                decl.identifier
                            ));
                        }
                        ensure_symbol(sym, &decl.identifier, &tty);
                    } else {
                        sym.erase(&decl.identifier);
                        sym.push(&decl.identifier, Variable(Some(Box::from(ty)), true, false));
                    }

                    morgana::write(self, &morg.out);
                    self.emit(ir_declare(&decl.identifier, &irty, &result));
                }

                (Expression(expr, is_eof), _) => {
                    let mut s = sym.clone();
                    let mut morg = MorganaExpr::new(MorganaContext::Declaration, &mut s);
                    let (result, ty) = morg.to_morgana(&expr.clone());

                    morgana::write(self, &morg.out);

                    if is_eof && ret.is_some() {
                        let ret_t = ret.clone().unwrap();
                        let (x, y) = (ret_t.to_string(), ty.to_string());

                        if x != y {
                            CompilerOutputs::fatal(format!(
                                "You can't return a {y}, because the expected type is {x}",
                            ));
                        }

                        make_defers(self, sym, &mut defers);

                        if ret_t.to_morgana() != Void {
                            morgana::ret(self, &result);
                        }
                    } else if is_eof {
                        if let Some(dest) = self.value_to.clone() {
                            self.emit(morgana::ir_store(&dest, &result));
                            make_defers(self, sym, &mut defers);
                        }
                    }
                }

                (Declaration(decl), Some(Node::Conditional(c))) => {
                    let c = c.clone();
                    self.nodes.remove(0);

                    let Some(ty) = c.ty.clone() else {
                        CompilerOutputs::fatal(format!(
                            "`{}` needs a value, but this `if` has none (it needs an `else`, and both branches must end in an expression without `;`)",
                            decl.identifier
                        ));
                        unreachable!();
                    };

                    if let Some(tty) = &decl._type {
                        let (x, y) = (tty.as_ref().to_string(), ty.to_string());
                        if x != y {
                            CompilerOutputs::fatal(format!(
                                "You defined {} as a {x} but was found {y}",
                                decl.identifier
                            ));
                        }
                    }

                    self.alloc_dest(&decl.identifier, &ty);
                    self.conditional(sym, &c, Some(decl.identifier.clone()));

                    match &decl._type {
                        Some(tty) => ensure_symbol(sym, &decl.identifier, tty),
                        None => {
                            sym.erase(&decl.identifier);
                            sym.push(&decl.identifier, Variable(Some(Box::from(ty)), true, false));
                        }
                    }
                }

                (Declaration(decl), _) => {
                    if let Some(tty) = &decl._type {
                        ensure_symbol(sym, &decl.identifier, tty);
                        self.emit(morgana::ir_alloc(&decl.identifier, &tty.to_morgana()));
                    }
                }

                (Node::Conditional(c), _) => {
                    let is_last = self.nodes.is_empty();

                    if c.ty.is_some() && is_last && ret.is_some() {
                        let ret_t = ret.clone().unwrap();
                        let ty = c.ty.clone().unwrap();

                        if ret_t.to_string() != ty.to_string() {
                            CompilerOutputs::fatal(format!(
                                "You can't return a {}, cuz was expected {}",
                                ty.to_string(),
                                ret_t.to_string()
                            ));
                        }

                        let result = fresh_name("t");
                        self.alloc_dest(&result, &ty);
                        self.conditional(sym, &c, Some(result.clone()));
                        make_defers(self, sym, &mut defers);

                        if ret_t.to_morgana() != Void {
                            morgana::ret(self, &result);
                        }
                    } else if c.ty.is_some() && is_last && self.value_to.is_some() {
                        let dest = self.value_to.clone();
                        self.conditional(sym, &c, dest);
                    } else {
                        self.conditional(sym, &c, None);
                    }
                }

                (Node::While(w), _) => self.while_loop(sym, &w),
                (Node::For(f), _) => self.for_loop(sym, &f),
                (Node::IncDec(step), _) => self.incdec(&step),
                (Node::Assign(a), _) => self.assign(sym, &a),

                (STMTReturn(expr), _) => {
                    let mut s = sym.clone();
                    let mut morg = MorganaExpr::new(MorganaContext::Declaration, &mut s);
                    let (result, ty) = morg.to_morgana(&expr.clone());

                    morgana::write(self, &morg.out);

                    let ret_t = ret.clone().unwrap();
                    let (x, y) = (ret_t.to_string(), ty.to_string());

                    if x != y {
                        CompilerOutputs::fatal(format!(
                            "You can't return a {y}, because the expected type is {x}",
                        ));
                    }

                    make_defers(self, sym, &mut defers);

                    if ret_t.to_morgana() != Void {
                        morgana::ret(self, &result);
                    }
                }

                (STMTReturnTuple(exprs), _) => {
                    let Some(ret_t) = ret.clone() else {
                        CompilerOutputs::fatal("`return` outside of a function");
                        unreachable!();
                    };

                    let irty = ret_t.to_morgana();
                    let MorganaType::Tuple(expected) = &irty else {
                        CompilerOutputs::fatal(format!(
                            "You can't return a tuple, because the expected type is {}",
                            ret_t.to_string()
                        ));
                        unreachable!();
                    };

                    if expected.len() != exprs.len() {
                        CompilerOutputs::fatal(format!(
                            "You returned {} values, but the expected type {} has {}",
                            exprs.len(),
                            ret_t.to_string(),
                            expected.len()
                        ));
                    }

                    let mut names = Vec::with_capacity(exprs.len());
                    for (i, (expr, want)) in exprs.iter().zip(expected).enumerate() {
                        let (value, ty) = self.eval(sym, expr);

                        if ty.to_morgana() != *want {
                            CompilerOutputs::fatal(format!(
                                "Tuple element {i}: you can't return a {}, because the expected type is {}",
                                ty.to_string(),
                                want.to_string()
                            ));
                        }

                        names.push(self.as_name(&value, &ty));
                    }

                    let result = fresh_name("t");
                    self.emit(morgana::ir_pack(&result, &irty, &names));

                    make_defers(self, sym, &mut defers);
                    morgana::ret(self, &result);
                }

                (Node::Break, _) => {
                    let Some((_, end)) = self.loops.last().cloned() else {
                        CompilerOutputs::fatal("`break` outside of a loop");
                        unreachable!();
                    };
                    self.emit(morgana::ir_jmp(&end));
                }

                (Node::Continue, _) => {
                    let Some((cont, _)) = self.loops.last().cloned() else {
                        CompilerOutputs::fatal("`continue` outside of a loop");
                        unreachable!();
                    };
                    self.emit(morgana::ir_jmp(&cont));
                }

                (Declare(decl), _) => match decl {
                    Extern::Variable(_, _) => todo!(),
                    Extern::Func(ret, params, identifier) => {
                        let mut args = vec![];
                        let mut carla_args = vec![];
                        for arg in &params {
                            for _ in 0..arg.ids.len() {
                                carla_args.push(arg.ty.clone());
                                args.push(arg.ty.to_morgana());
                            }
                        }
                        morgana::_extern(self, ret.to_morgana(), identifier.clone(), args);
                    }
                },

                (Unfoldable(decl), _) => match decl {
                    Unfold::Func(ret, params, identifier) => {
                        let mut args = vec![];
                        let mut carla_args = vec![];
                        for arg in &params {
                            for _ in 0..arg.ids.len() {
                                carla_args.push(arg.ty.clone());
                                args.push(arg.ty.to_morgana());
                            }
                        }
                        morgana::unfold(self, ret.to_morgana(), identifier.clone(), args);
                    }
                },

                (Extension(ty, block), _) => {
                    let mut cgen = Codegen::new(block);
                    cgen.extending = Some(ty.clone());
                    let str = cgen.generate(sym, None);
                    morgana::write(self, &str);
                }

                _ => {}
            }
        }

        make_defers(self, sym, &mut defers);

        self.code.clone()
    }
}


impl Codegen {
    fn emit<T: ToString>(&mut self, line: T) {
        self.append(line);
        self.append("\n");
    }

    fn alloc_dest(&mut self, name: &str, ty: &CarlaType) {
        let irty = ty.to_morgana();

        if matches!(irty, MorganaType::Tuple(_)) {
            CompilerOutputs::fatal(
                "an `if` that produces a struct/tuple value is not supported yet (`store` is scalar only)",
            );
        } else {
            self.emit(morgana::ir_alloc(name, &irty));
        }
    }

    fn as_name(&mut self, v: &str, ty: &CarlaType) -> String {
        if !is_literal(v) {
            return v.to_string();
        }

        let t = fresh_name("c");
        self.emit(ir_declare(&t, &ty.to_morgana(), v));
        t
    }

    fn eval(&mut self, sym: &Symbols, expr: &Compiled) -> (String, CarlaType) {
        let mut s = sym.clone();
        let mut morg = MorganaExpr::new(MorganaContext::Declaration, &mut s);
        let (value, ty) = morg.to_morgana(expr);
        morgana::write(self, &morg.out);
        (value, ty)
    }

    fn eval_cond(&mut self, sym: &Symbols, expr: &Compiled) -> String {
        let (value, ty) = self.eval(sym, expr);
        self.as_name(&value, &ty)
    }

    fn sub(
        &self,
        sym: &mut Symbols,
        nodes: &[Node],
        scoped: bool,
        value_to: Option<String>,
    ) -> String {
        let mut cg = Codegen::new(nodes.to_vec());

        cg.extending = self.extending.clone();
        cg.loops = self.loops.clone();
        cg.value_to = value_to;
        cg.ret = self.ret.clone();

        if scoped {
            sym.push_scope();
        }

        let code = cg.generate(sym, self.ret.clone());

        if scoped {
            sym.pop_scope();
        }

        code
    }

    fn loop_body(
        &self,
        sym: &mut Symbols,
        nodes: &[Node],
        cont: String,
        brk: String,
    ) -> String {
        let mut cg = Codegen::new(nodes.to_vec());

        cg.extending = self.extending.clone();
        cg.loops = self.loops.clone();
        cg.loops.push((cont, brk));
        cg.ret = self.ret.clone();

        sym.push_scope();

        let code = cg.generate(sym, self.ret.clone());

        sym.pop_scope();

        code
    }

    fn declare_binding(sym: &mut Symbols, b: &Binding) {
        for (name, ty) in b.names.iter().zip(&b.types) {
            if let Some(name) = name {
                ensure_symbol(sym, name, ty);
            }
        }
    }

    fn bind(&mut self, sym: &Symbols, b: &Binding) {
        let (value, _) = self.eval(sym, &b.value);

        if b.names.len() == 1 {
            if let Some(name) = &b.names[0] {
                self.emit(ir_declare(name, &b.types[0].to_morgana(), &value));
            }
            return;
        }

        let types: Vec<MorganaType> = b.types.iter().map(|t| t.to_morgana()).collect();
        for line in morgana::ir_destructure(&value, &b.names, &types, &b.offsets) {
            self.emit(line);
        }
    }

    fn conditional(&mut self, sym: &mut Symbols, c: &Conditional, dest: Option<String>) {
        if let Some(b) = c.init.as_ref().filter(|b| !b.scoped) {
            Self::declare_binding(sym, b);
        }

        sym.push_scope();
        if let Some(b) = c.init.as_ref().filter(|b| b.scoped) {
            Self::declare_binding(sym, b);
        }

        if let Some(b) = &c.init {
            self.bind(sym, b);
        }

        if c.init.is_none() {
            if let Compiled::Const(Expr::Boolean(taken, _)) = &c.cond {
                let branch = if *taken { Some(&c.then) } else { c.otherwise.as_ref() };
                if let Some(nodes) = branch {
                    let code = self.sub(sym, nodes, true, dest);
                    self.append(code);
                }
                sym.pop_scope();
                return;
            }
        }

        let cond = self.eval_cond(sym, &c.cond);
        let end = fresh_name("L");

        match &c.otherwise {
            None => {
                self.emit(morgana::ir_jz(&cond, &end));
                let code = self.sub(sym, &c.then, true, dest);
                self.append(code);
            }
            Some(other) => {
                let otherwise = fresh_name("L");
                self.emit(morgana::ir_jz(&cond, &otherwise));
                let code = self.sub(sym, &c.then, true, dest.clone());
                self.append(code);
                self.emit(morgana::ir_jmp(&end));
                self.emit(morgana::ir_label(&otherwise));
                let code = self.sub(sym, other, true, dest);
                self.append(code);
            }
        }

        self.emit(morgana::ir_label(&end));
        sym.pop_scope();
    }

    fn while_loop(&mut self, sym: &mut Symbols, w: &WhileLoop) {
        if let Some(b) = w.init.as_ref().filter(|b| !b.scoped) {
            Self::declare_binding(sym, b);
        }

        sym.push_scope();
        if let Some(b) = w.init.as_ref().filter(|b| b.scoped) {
            Self::declare_binding(sym, b);
        }

        let (top, end) = (fresh_name("L"), fresh_name("L"));
        self.emit(morgana::ir_label(&top));

        if let Some(b) = &w.init {
            self.bind(sym, b);
        }

        if !matches!(&w.cond, Compiled::Const(Expr::Boolean(true, _))) {
            let cond = self.eval_cond(sym, &w.cond);
            self.emit(morgana::ir_jz(&cond, &end));
        }

        let code = self.loop_body(sym, &w.body, top.clone(), end.clone());
        self.append(code);

        self.emit(morgana::ir_jmp(&top));
        self.emit(morgana::ir_label(&end));
        sym.pop_scope();
    }

    fn for_loop(&mut self, sym: &mut Symbols, f: &ForLoop) {
        if let ForLoop::C { bind: Some(b), .. } = f {
            if !b.scoped {
                Self::declare_binding(sym, b);
            }
        }

        sym.push_scope();

        match f {
            ForLoop::C {
                init,
                bind,
                cond,
                step,
                body,
            } => {
                if let Some(b) = bind {
                    if b.scoped {
                        Self::declare_binding(sym, b);
                    }
                    self.bind(sym, b);
                }

                let code = self.sub(sym, init, false, None);
                self.append(code);

                let (top, cont, end) = (fresh_name("L"), fresh_name("L"), fresh_name("L"));
                self.emit(morgana::ir_label(&top));

                if let Some(cond) = cond {
                    if !matches!(cond, Compiled::Const(Expr::Boolean(true, _))) {
                        let v = self.eval_cond(sym, cond);
                        self.emit(morgana::ir_jz(&v, &end));
                    }
                }

                let code = self.loop_body(sym, body, cont.clone(), end.clone());
                self.append(code);

                self.emit(morgana::ir_label(&cont));
                if let Some(step) = step {
                    match step.as_ref() {
                        Node::IncDec(s) => self.incdec(s),
                        Node::Assign(a) => self.assign(sym, a),
                        _ => {}
                    }
                }

                self.emit(morgana::ir_jmp(&top));
                self.emit(morgana::ir_label(&end));
            }

            ForLoop::Range {
                var,
                ty,
                start,
                end,
                inclusive,
                body,
            } => {
                let (first, _) = self.eval(sym, start);
                let (last, _) = self.eval(sym, end);

                let irty = ty.to_morgana();
                let signed = is_signed(&irty);

                let last = if is_literal(&last) {
                    last
                } else {
                    let lim = fresh_name("lim");
                    self.emit(ir_declare(&lim, &irty, &last));
                    lim
                };

                let counter = match var {
                    Some(name) => {
                        ensure_symbol(sym, name, ty);
                        name.clone()
                    }
                    None => fresh_name("i"),
                };
                self.emit(ir_declare(&counter, &irty, &first));

                let (top, cont, done) = (fresh_name("L"), fresh_name("L"), fresh_name("L"));
                self.emit(morgana::ir_label(&top));

                let (cur, flag) = (fresh_name("t"), fresh_name("t"));
                self.emit(format!("{cur} = load {counter}"));
                self.emit(morgana::ir_binary(
                    &flag,
                    if *inclusive { "le" } else { "lt" },
                    signed,
                    &cur,
                    &last,
                ));
                self.emit(morgana::ir_jz(&flag, &done));

                let code = self.loop_body(sym, body, cont.clone(), done.clone());
                self.append(code);

                self.emit(morgana::ir_label(&cont));
                let cur = fresh_name("t");
                self.emit(format!("{cur} = load {counter}"));

                if *inclusive {
                    let reached = fresh_name("t");
                    self.emit(morgana::ir_binary(&reached, "eq", signed, &cur, &last));
                    self.emit(morgana::ir_jnz(&reached, &done));
                }

                let next = fresh_name("t");
                self.emit(morgana::ir_binary(&next, "add", signed, &cur, "1"));
                self.emit(morgana::ir_store(&counter, &next));

                self.emit(morgana::ir_jmp(&top));
                self.emit(morgana::ir_label(&done));
            }
        }

        sym.pop_scope();
    }

    fn assign(&mut self, sym: &Symbols, a: &Assign) {
        match &a.target {
            Target::Var(name) => {
                let (value, _) = self.eval(sym, &a.value);
                self.emit(morgana::ir_store(name, &value));
            }
            Target::Deref(ptr) => {
                let (addr, _) = self.eval(sym, ptr);
                let (value, _) = self.eval(sym, &a.value);
                self.emit(morgana::ir_store_deref(&a.ty.to_morgana(), &addr, &value));
            }
        }
    }

    fn incdec(&mut self, s: &IncDec) {
        let signed = is_signed(&s.ty.to_morgana());
        let (cur, next) = (fresh_name("t"), fresh_name("t"));

        self.emit(format!("{cur} = load {}", s.name));
        self.emit(morgana::ir_binary(
            &next,
            if s.increment { "add" } else { "sub" },
            signed,
            &cur,
            "1",
        ));
        self.emit(morgana::ir_store(&s.name, &next));
    }

}
