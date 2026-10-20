use crate::{
    compiler::{
        iterator::ContextIterator,
        nesting::Context::{self, Block, Common},
        node::Node::{self, Expression, Nop},
        patterns,
        symbols::{Symbol, Symbols},
    },
    utils::compiler_outputs::CompilerOutputs,
};

macro_rules! last {
    ($iterators:expr) => {
        $iterators.last_mut().unwrap()
    };
}

pub fn work(ctx: &Vec<Context>, syms: &mut Symbols) -> Vec<Node> {
    let mut nodes = vec![];
    let mut iterators = vec![ContextIterator::new(ctx.as_slice(), 0)];
    syms.push_scope();

    let mut scopes = 0;
    while iterators.len() > 0 && last!(iterators).remaining() > 0 {
        let iterator = last!(iterators);
        let Some(node) = iterator.next() else {
            CompilerOutputs::fatal("Context not found");
            unreachable!()
        };

        match node {
            Block(_, ctx) => {
                syms.push_scope();
                iterators.push(ContextIterator::new(ctx.as_slice(), 0));
                scopes += 1;
            }

            Common(token) => {
                let data = patterns::handler(iterator, syms, &token).unwrap_or(Nop);

                if let (Expression(expr, _), Some(Node::Declaration(var))) = (&data, nodes.last()) {
                    if var._type.is_none() {
                        let ty = Box::new(expr.expr().ty());
                        let name = &var.identifier;
                        if let Some(Symbol::Variable(_, a, b)) = syms.lookup(name).cloned() {
                            syms.erase(name);
                            syms.push(name, Symbol::Variable(Some(ty), a, b));
                        }
                    }
                }

                nodes.push(data);
            }
        }

        if last!(iterators).remaining() == 0 {
            iterators.pop();
            if scopes > 0 {
                scopes -= 1;
                syms.pop_scope();
            }
        }
    }

    nodes
}
