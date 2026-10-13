use crate::{
    compiler::{
        iterator::ContextIterator,
        nesting::Context::{self, Block, Common},
        node::Node::{self, Nop},
        patterns,
        symbols::Symbols,
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

    while iterators.len() > 0 && last!(iterators).remaining() > 0 {
        let iterator = last!(iterators);
        let Some(node) = iterator.next() else {
            CompilerOutputs::fatal("Context not found");
            unreachable!()
        };

        match node {
            Block(_, ctx) => {
                syms.push_scope();
                iterators.push(ContextIterator::new(ctx.as_slice(), 0))
            }

            Common(token) => {
                let data = patterns::handler(iterator, syms, &token).unwrap_or(Nop);
                nodes.push(data);
            }
        }

        if last!(iterators).remaining() == 0 {
            iterators.pop();
            syms.pop_scope();
        }
    }

    nodes
}
