use std::collections::HashMap;

use crate::compiler::{
    codegen::Codegen,
    patterns::{expressions::ExprId, types::CarlaType},
    symbols::Symbols,
};

#[derive(Debug, Clone, PartialEq)]
pub enum MorganaType {
    Tuple(Vec<MorganaType>),
    UInteger(i32),
    Integer(i32),
    Void,
    Ptr,
}

impl MorganaType {
    pub fn to_string(&self) -> String {
        match self {
            MorganaType::Integer(bits) => format!("i{bits}"),
            MorganaType::UInteger(bits) => format!("u{bits}"),
            MorganaType::Void => "void".into(),
            MorganaType::Ptr => "ptr".into(),
            MorganaType::Tuple(types) => format!(
                "{{ {} }}",
                types
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum MorganaContext {
    Declaration,
    Arguments,

    #[default]
    Unknown,
}

#[derive(Debug)]
pub struct MorganaExpr<'a> {
    pub syms: &'a mut Symbols,
    pub ctx: MorganaContext,
    pub vals: HashMap<ExprId, (String, CarlaType)>,
    pub out: String,
    pub tmp: usize,
}

pub trait MorganaGenerator<T> {
    fn to_morgana(&mut self, data: &T) -> (String, CarlaType);
}

impl<'a> MorganaExpr<'a> {
    pub fn new(ctx: MorganaContext, syms: &'a mut Symbols) -> Self {
        Self {
            syms,
            ctx,
            vals: HashMap::new(),
            out: String::new(),
            tmp: 0,
        }
    }
}

pub fn _extern(ctx: &mut Codegen, ret: MorganaType, identifier: String, args: Vec<MorganaType>) {
    ctx.append(format!(
        "extern {} {identifier}({})\n",
        ret.to_string(),
        args.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", "),
    ));
}

pub fn unfold(ctx: &mut Codegen, ret: MorganaType, identifier: String, args: Vec<MorganaType>) {
    ctx.append(format!(
        "unfold {} {identifier}({})\n",
        ret.to_string(),
        args.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", "),
    ));
}

pub fn function(
    ctx: &mut Codegen,
    ret: MorganaType,
    identifier: String,
    args: Vec<(String, MorganaType)>,
    body: String,
) {
    ctx.append(format!(
        "\n{} {identifier}({}) {{\n{}{body}}}\n",
        ret.to_string(),
        args.iter()
            .map(|x| x.1.to_string())
            .collect::<Vec<String>>()
            .join(", "),
        if args.len() == 0 {
            String::new()
        } else {
            [
                "@_ : ",
                &args
                    .iter()
                    .map(|x| x.0.clone())
                    .collect::<Vec<String>>()
                    .join(", "),
                "\n",
            ]
            .concat()
        }
    ));
}

pub fn get_function_ret(
    ctx: &mut Codegen,
    identifier: &String,
    function: &String,
    arguments: &String,
) {
    ctx.append(format!("{identifier} = call {function}({arguments})\n"));
}

pub fn write(ctx: &mut Codegen, content: &String) {
    if !content.is_empty() {
        ctx.append(format!("{content}"));
    }
}

pub fn rename(ctx: &mut Codegen, lhs: &String, rhs: &String) {
    ctx.append(format!("{lhs} = move {rhs}\n"));
}

pub fn ret(ctx: &mut Codegen, data: &String) {
    ctx.append(format!("ret {data}\n"));
}

pub fn ir_alloc(dst: &str, ty: &MorganaType) -> String {
    format!("{dst} = alloc {}", ty.to_string())
}

pub fn ir_store(dst: &str, value: &str) -> String {
    format!("store {dst} {value}")
}

pub fn ir_pack(dst: &str, ty: &MorganaType, values: &[String]) -> String {
    format!("{dst} = pack {} {}", ty.to_string(), values.join(", "))
}

pub fn ir_store_deref(ty: &MorganaType, ptr: &str, value: &str) -> String {
    format!("storep {} {ptr}, {value}", ty.to_string())
}

pub fn ir_const(dst: &str, ty: &MorganaType, value: &str) -> String {
    format!("{}\n{}", ir_alloc(dst, ty), ir_store(dst, value))
}

pub fn ir_move(dst: &str, src: &str) -> String {
    format!("{dst} = move {src}")
}

pub fn ir_label(name: &str) -> String {
    format!("::{name}::")
}

pub fn ir_jmp(name: &str) -> String {
    format!("br {name}")
}

pub fn ir_jz(cond: &str, name: &str) -> String {
    format!("brez {cond}, {name}")
}
/// `brnz cond, L` — salta se cond != 0
pub fn ir_jnz(cond: &str, name: &str) -> String {
    format!("brnz {cond}, {name}")
}

pub fn ir_binary(dst: &str, op: &str, signed: bool, a: &str, b: &str) -> String {
    let needs_sign = matches!(op, "div" | "rem" | "lt" | "le" | "gt" | "ge");
    let op = if needs_sign {
        format!("{}{op}", if signed { 's' } else { 'u' })
    } else {
        op.to_string()
    };
    format!("{dst} = {op} {a}, {b}")
}

pub fn ir_extract(dst: &str, ty: &MorganaType, src: &str, index: usize) -> String {
    format!("{dst} = extract {} {src}, {index}", ty.to_string())
}

/// `dst = ptr_to src` — address of a variable
pub fn ir_addr_of(dst: &str, src: &str) -> String {
    format!("{dst} = ptr_to {src}")
}

/// `dst = deref ty ptr` — load a `ty` from the address `ptr`
pub fn ir_deref(dst: &str, ty: &MorganaType, ptr: &str) -> String {
    format!("{dst} = deref {} {ptr}", ty.to_string())
}

/// Destructures the tuple value `src`: `names[i]` receives the element of type `types[i]`
/// that lives `offsets[i]` bytes after the start of the tuple. `None` is `_` and emits nothing.
///
/// The tuple is kept in a local, its address is taken once (`ptr_to`, parked in a pointer
/// variable) and every element is
/// read with plain pointer arithmetic (`add ptr, offset` + `deref`), exactly what `p + n` and
/// `p.*` produce in expressions. The element is then declared like any other local:
/// `alloc` + `store` for scalars (variables are slots read with `load`), `move` for a nested tuple.
pub fn ir_destructure(
    src: &str,
    names: &[Option<String>],
    types: &[MorganaType],
    offsets: &[i128],
) -> Vec<String> {
    let mut lines = Vec::new();

    if names.iter().all(|n| n.is_none()) {
        return lines;
    }

    let id = || crate::utils::uuid::gen_hex(16 * 3);

    let tuple = format!("tp{}", id());
    let addr = format!("tr{}", id());
    let base = format!("tb{}", id());
    lines.push(ir_move(&tuple, src));
    lines.push(ir_addr_of(&addr, &tuple));

    // The result of a `ptr_to` can't be the operand of an `add` directly (the backend reads it
    // as a slot, not as an address; same limit as `&x + n` in expressions). So the address is
    // parked in a pointer variable and `load`ed for every element.
    lines.push(ir_const(&base, &MorganaType::Ptr, &addr));

    for ((name, ty), offset) in names.iter().zip(types).zip(offsets) {
        let Some(name) = name else { continue };

        let cur = format!("tc{}", id());
        lines.push(format!("{cur} = load {base}"));

        // the first element sits at offset 0: the base pointer already is its address
        let at = if *offset == 0 {
            cur
        } else {
            let a = format!("ta{}", id());
            lines.push(ir_binary(&a, "add", false, &cur, &offset.to_string()));
            a
        };

        let tmp = format!("tv{}", id());
        lines.push(ir_deref(&tmp, ty, &at));
        lines.push(match ty {
            MorganaType::Tuple(_) => ir_move(name, &tmp),
            _ => ir_const(name, ty, &tmp),
        });
    }

    lines
}
