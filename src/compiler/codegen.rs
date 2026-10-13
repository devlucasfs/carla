use crate::{
    compiler::{
        node::Node::{self, Declaration, Declare, Function, Nop},
        patterns::r#extern::Extern::{self, Func},
        symbols::Symbol::Variable,
    },
    morgana,
};

#[derive(Debug, Clone, Default)]
pub struct Codegen {
    pub nodes: Vec<Node>,
    pub code: String,
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

    pub fn generate(&mut self) -> String {
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

                    morgana::function(self, decl._type.to_morgana(), decl.identifier, args);
                }

                (Declare(decl), _) => match decl {
                    Extern::Variable(_, _) => todo!(),
                    Extern::Func(ret, params, identifier) => {
                        let mut args = vec![];
                        for arg in &params {
                            for i in 0..arg.ids.len() {
                                args.push(arg.ty.to_morgana());
                            }
                        }
                        morgana::_extern(self, ret.to_morgana(), identifier, args);
                    }
                },

                _ => {}
            }
        }

        self.code.clone()
    }
}
