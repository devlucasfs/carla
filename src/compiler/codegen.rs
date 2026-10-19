use crate::{
    compiler::{
        node::Node::{self, Declaration, Declare, Expression, Function, Nop},
        patterns::{expressions::Expr, r#extern::Extern, lambda::LambdaRunner, types::CarlaType},
        symbols::{Symbol, Symbols},
        visitor,
    },
    morgana,
    utils::{compiler_outputs::CompilerOutputs, uuid},
};

#[derive(Debug, Clone, Default)]
pub struct Codegen {
    pub nodes: Vec<Node>,
    pub code: String,
}

fn check_numeric(i: &i128, ty: &CarlaType, syms: &mut Symbols) -> CarlaType {
    let int = CarlaType::from("int", syms);
    if let Some(base) = &ty.base {
        return match base.symbol {
            Symbol::BType(morgana::MorganaType::Integer(_)) => ty.clone(),
            Symbol::BType(morgana::MorganaType::UInteger(_)) if *i >= 0 => ty.clone(),
            _ => int,
        };
    }
    int
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

    pub fn parse_expr_to_morgana(
        &mut self,
        syms: &mut Symbols,
        before: &mut String,
        expr: &Expr,
        expected: &CarlaType,
    ) -> (String, CarlaType) {
        match expr {
            Expr::Integer(i) => (i.to_string(), check_numeric(&i, expected, syms)),
            Expr::Pointer(id) => match syms.lookup(id) {
                Some(Symbol::Variable(ty, _, func)) if !*func => {
                    let morg = uuid::gen_hex(64);
                    before.push_str(&format!("{morg} = ptr_to {id}"));
                    let mut cp = ty.as_ref().clone();
                    cp.levels += 1;
                    (morg, cp)
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    pub fn generate(&mut self, sym: &mut Symbols) -> String {
        while self.nodes.len() > 0 {
            match (self.nodes.remove(0), (self.nodes.clone()).first()) {
                (Nop, _) => {}
                (Declaration(decl), Some(Function(func))) => {
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
                            let str = cgen.generate(sym);
                            sym.pop_scope();
                            str
                        }
                        _ => todo!(),
                    };

                    morgana::function(self, decl._type.to_morgana(), decl.identifier, args, body);
                }

                (Declaration(decl), Some(Expression(expr, _))) => match expr {
                    Expr::Call(func, params) => {
                        let mut before = String::new();
                        let mut after = String::new();

                        let function = sym.lookup(func).unwrap();
                        let (_ids, types) = match function {
                            Symbol::ExternFunction(_, args) => {
                                let mut ids = vec![];
                                let mut types = vec![];
                                for arg in args {
                                    for id in &arg.ids {
                                        ids.push(id);
                                        types.push(arg.ty.as_ref().clone());
                                    }
                                }
                                (ids, types)
                            }
                            x => {
                                println!("entrou aqui em {} com o {:?}", func, x);
                                todo!()
                            }
                        };

                        for (i, node) in params.iter().enumerate() {
                            let Expression(expr, eof) = node else {
                                unreachable!();
                            };

                            let ty = &types[i];
                            let (morg, check) =
                                self.parse_expr_to_morgana(sym, &mut before, expr, ty);
                            after.push_str(&morg);

                            if *ty != check {
                                CompilerOutputs::fatal(format!(
                                    "Was expected {} but was found {} when you are calling {}.",
                                    ty.to_string(),
                                    check.to_string(),
                                    func,
                                ));
                            }

                            if !eof {
                                after.push_str(", ");
                            }
                        }

                        morgana::write(self, &before);
                        morgana::get_function_ret(self, &decl.identifier, &func, &after);
                    }
                    _ => todo!(),
                },

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

                _ => {}
            }
        }

        self.code.clone()
    }
}
