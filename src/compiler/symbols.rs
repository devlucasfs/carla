use crate::{
    compiler::patterns::{lambda::Arg, types::CarlaType},
    morgana::MorganaType,
    utils::compiler_outputs::CompilerOutputs,
};

type Stack<T> = Vec<T>;

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    BType(MorganaType),
    TypeAlias(Box<CarlaType>),
    Variable(Option<Box<CarlaType>>, bool, bool),
    ExternFunction(Box<CarlaType>, Vec<Arg>),
    UnfoldFunction(Box<CarlaType>, Vec<Arg>),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Symbols {
    stack: Stack<Vec<(String, Symbol)>>,
    extending: Option<CarlaType>,
}

impl Symbols {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            ..Default::default()
        }
    }

    pub fn extending(&self) -> Option<&CarlaType> {
        self.extending.as_ref()
    }

    pub fn enter_extension(&mut self, ty: CarlaType) {
        self.extending = Some(ty);
    }

    pub fn get_extended(&self) -> Option<CarlaType> {
        self.extending.clone()
    }

    pub fn temporary<T>(
        &mut self,
        vars: &[(String, CarlaType)],
        f: impl FnOnce(&mut Symbols) -> T,
    ) -> T {
        self.push_scope();
        for (name, ty) in vars {
            self.push(
                name,
                Symbol::Variable(Some(Box::new(ty.clone())), true, false),
            );
        }
        let out = f(self);
        self.pop_scope();
        out
    }

    pub fn leave_extension(&mut self) {
        self.extending = None;
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
        let id = id.to_string();

        for scope in self.stack.iter_mut().rev() {
            if let Some(pos) = scope.iter().position(|(name, _)| *name == id) {
                scope.remove(pos);
                return;
            }
        }
    }
}
