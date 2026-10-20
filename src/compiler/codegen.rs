use crate::{
    compiler::{
        node::Node::{
            self, Declaration, Declare, Expression, Function, Nop, STMTDefer, Unfoldable,
        },
        patterns::{
            expressions::{Compiled, Expr, Operand, Step, StepOp},
            r#extern::Extern,
            lambda::LambdaRunner,
            types::CarlaType,
            unfold::Unfold,
        },
        symbols::{
            Symbol::{ExternFunction, Variable},
            Symbols,
        },
        visitor,
    },
    morgana::{
        self, MorganaContext, MorganaExpr, MorganaGenerator,
        MorganaType::{self, Void},
    },
    utils::{compiler_outputs::CompilerOutputs, uuid},
};

#[derive(Debug, Clone, Default)]
pub struct Codegen {
    pub nodes: Vec<Node>,
    pub code: String,
}

impl<'a> MorganaExpr<'a> {
    fn fresh(&mut self) -> String {
        ["t", &uuid::gen_hex(16 * 3)].concat()
    }

    fn line(&mut self, s: String) {
        self.out.push_str(&s);
        self.out.push('\n');
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
                    ExternFunction(ty, _) => ty.as_ref().clone(),
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

                (t, ty)
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
        let mut defers = vec![];

        let make_defers = |r: &mut Codegen, sym: &mut Symbols, defers: &mut Vec<Compiled>| {
            while let Some(defer) = defers.pop() {
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
                    let mut args = vec![];

                    for arg in &func.args {
                        for i in 0..arg.ids.len() {
                            args.push((arg.ids[i].clone(), arg.ty.to_morgana()));
                        }
                    }

                    let body = match &func.run {
                        LambdaRunner::Scope(ctx) => {
                            sym.push_scope();
                            let nodes = visitor::work(&ctx, sym);
                            let mut cgen = Codegen::new(nodes);
                            let str = cgen
                                .generate(sym, Some(decl._type.clone().unwrap().as_ref().clone()));
                            sym.pop_scope();
                            str
                        }
                        LambdaRunner::Expr(expr) => {
                            sym.push_scope();
                            let nodes = vec![Expression(expr.clone(), true)];
                            let mut cgen = Codegen::new(nodes);
                            let str = cgen
                                .generate(sym, Some(decl._type.clone().unwrap().as_ref().clone()));
                            sym.pop_scope();
                            str
                        }
                        _ => todo!(),
                    };

                    let ctype = decl._type.unwrap();
                    morgana::function(self, ctype.to_morgana(), decl.identifier, args, body);
                }

                (Declaration(decl), Some(Expression(expr, _))) => {
                    let mut s = sym.clone();
                    let mut morg = MorganaExpr::new(MorganaContext::Declaration, &mut s);
                    let (result, ty) = morg.to_morgana(&expr.clone());
                    self.nodes.remove(0);

                    if let Some(tty) = decl._type {
                        let (x, y) = (tty.as_ref().to_string(), ty.to_string());
                        if x != y {
                            CompilerOutputs::fatal(format!(
                                "You defined {} as a {x} but was found {y}",
                                decl.identifier
                            ));
                        }
                    } else {
                        sym.erase(&decl.identifier);
                        sym.push(&decl.identifier, Variable(Some(Box::from(ty)), true, false));
                    }

                    morgana::write(self, &morg.out);
                    morgana::rename(self, &decl.identifier, &result);
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
                                "You can't return a {y}, cuz was expected {x}",
                            ));
                        }

                        make_defers(self, sym, &mut defers);

                        if ret_t.to_morgana() != Void {
                            morgana::ret(self, &result);
                        }
                    }
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

                _ => {}
            }
        }

        make_defers(self, sym, &mut defers);

        self.code.clone()
    }
}
