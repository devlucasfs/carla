use crate::compiler::{
    iterator::ContextIterator,
    nesting::Context::{self, Block, Common},
    node::Node,
    patterns::{declaration, r#extern, lambda, types, using},
    scanner::{
        Keywords::{Extern, Fn, Using},
        PBB::Brace,
        Token,
        TokenKind::Keyword,
    },
    symbols::Symbols,
};

pub fn work(
    _: &Token,
    syms: &mut Symbols,
    iterator: &mut ContextIterator<'_>,
) -> Result<Node, String> {
    let err = "After extend you need a type";
    let Some(Common(first)) = iterator.next() else {
        return Err(err.into());
    };
    if !types::starts_with(&first.kind) {
        return Err(err.into());
    }

    let Node::InternalType(ty) = types::work(first, syms, iterator)? else {
        unreachable!()
    };

    syms.enter_extension(ty.clone());

    let Some(Block(pbb, body)) = iterator.next() else {
        return Err("After the extend type you need a {} block".into());
    };

    if *pbb != Brace {
        return Err("After the extend type you need a {} block".into());
    }

    let nodes = parse_body(body.as_slice(), syms);
    syms.leave_extension();

    Ok(Node::Extension(ty, nodes?))
}

fn parse_body(body: &[Context], syms: &mut Symbols) -> Result<Vec<Node>, String> {
    let mut sub = ContextIterator::new(body, 0);
    let mut nodes = vec![];
    while let Some(ctx) = sub.next() {
        let Common(tk) = ctx else {
            return Err("Only `using` and `fn` are allowed inside an extend block".into());
        };

        if types::starts_with(&tk.kind) {
            let node = declaration::work(tk, syms, &mut sub);
            if let Err(err) = node {
                return Err(format!("Invalid declaration: {}", err));
            }
            nodes.push(node.unwrap());
            continue;
        }

        let node = match tk.kind {
            Keyword(Fn) => lambda::work_in(tk, syms, &mut sub, true)?,
            Keyword(Using) => {
                _ = using::work(tk, syms, &mut sub);
                continue;
            }
            Keyword(Extern) => {
                _ = r#extern::work(tk, syms, &mut sub);
                continue;
            }
            _ => {
                return Err(format!(
                    "Only `using` and `fn` are allowed inside an extend block, found {:?}",
                    tk.kind
                ));
            }
        };
        nodes.push(node);
    }
    Ok(nodes)
}
