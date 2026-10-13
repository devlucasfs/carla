use crate::{
    compiler::patterns::types::CarlaType, morgana::MorganaType,
    utils::compiler_outputs::CompilerOutputs,
};

type Stack<T> = Vec<T>;

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    BType(MorganaType),
    Variable(Box<CarlaType>, bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbols {
    stack: Stack<Vec<(String, Symbol)>>,
}

impl Symbols {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn push_scope(&mut self) {
        self.stack.push(vec![]);
    }

    pub fn pop_scope(&mut self) {
        self.stack.push(vec![]);
    }

    pub fn lookup<T: ToString>(&self, id: T) -> Option<&Symbol> {
        let id = id.to_string();
        for scope in self.stack.iter().rev() {
            for (name, symbol) in scope {
                if name == &id {
                    return Some(symbol);
                }
            }
        }
        None
    }

    pub fn push<T: ToString>(&mut self, id: T, sym: Symbol) {
        let Some(top) = self.stack.last_mut() else {
            CompilerOutputs::fatal_internal("Can't reach the symbol stack top");
            unreachable!()
        };

        top.push((id.to_string(), sym));
    }

    pub fn erase<T: ToString>(&mut self, id: T) {
        let Some(top) = self.stack.last_mut() else {
            CompilerOutputs::fatal_internal("Can't reach the symbol stack top");
            unreachable!()
        };

        top.pop_if(|x| x.0 == id.to_string());
    }
}
