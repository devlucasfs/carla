use crate::{
    compiler::scanner::{
        PBB, Token,
        TokenKind::{Close, Open},
    },
    utils::compiler_outputs::CompilerOutputs,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Context {
    Common(Token),
    Block(PBB, Vec<Context>),
}

pub fn work(tokens: Vec<Token>) -> Vec<Context> {
    let mut stack: Vec<(Option<PBB>, Vec<Context>)> = vec![(None, vec![])];

    for token in tokens {
        match token.kind {
            Open(ppb) => {
                stack.push((Some(ppb), vec![]));
            }

            Close(ppb) => {
                if stack.len() <= 1 {
                    CompilerOutputs::fatal(
                        "Unexpected CLOSE_CONTEXT with no matching OPEN_CONTEXT",
                    );
                }

                let (open_ppb, _) = stack.last().unwrap();

                if *open_ppb != Some(ppb) {
                    CompilerOutputs::fatal("CLOSE_CONTEXT does not match OPEN_CONTEXT");
                }

                let (open_ppb, block) = stack.pop().unwrap();

                stack
                    .last_mut()
                    .unwrap()
                    .1
                    .push(Context::Block(open_ppb.unwrap(), block));
            }

            _ => {
                stack.last_mut().unwrap().1.push(Context::Common(token));
            }
        }
    }

    if stack.len() != 1 {
        CompilerOutputs::fatal("Unclosed block(s) detected at end of input");
    }

    stack.pop().unwrap().1
}
