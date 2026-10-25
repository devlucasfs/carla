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

pub fn infer_declaration(syms: &mut Symbols, prev: Option<&Node>, data: &Node) {
    let inferred = match data {
        Expression(expr, _) => Some(expr.expr().ty()),
        Node::Conditional(c) => c.ty.clone(),
        _ => None,
    };

    if let (Some(ty), Some(Node::Declaration(var))) = (inferred, prev) {
        if var._type.is_none() {
            let ty = Box::new(ty);
            let name = &var.identifier;
            if let Some(Symbol::Variable(_, a, b)) = syms.lookup(name).cloned() {
                syms.erase(name);
                syms.push(name, Symbol::Variable(Some(ty), a, b));
            }
        }
    }
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

                infer_declaration(syms, nodes.last(), &data);
                nodes.push(data);
            }
        }

        while iterators.last().is_some_and(|i| i.remaining() == 0) {
            iterators.pop();
            if scopes > 0 {
                scopes -= 1;
                syms.pop_scope();
            }
        }
    }

    nodes
}

pub fn block(ctx: &Vec<Context>, syms: &mut Symbols) -> Vec<Node> {
    let nodes = work(ctx, syms);
    syms.pop_scope();
    nodes
}
