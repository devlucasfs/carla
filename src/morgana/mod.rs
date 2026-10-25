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

/// `dst = alloc ty`
pub fn ir_alloc(dst: &str, ty: &MorganaType) -> String {
    format!("{dst} = alloc {}", ty.to_string())
}

/// `store dst value` — value é um nome ou um inteiro
pub fn ir_store(dst: &str, value: &str) -> String {
    format!("store {dst} {value}")
}

/// Constante: NÃO existe `move 0`. Aloca e guarda (duas linhas).
pub fn ir_const(dst: &str, ty: &MorganaType, value: &str) -> String {
    format!("{}\n{}", ir_alloc(dst, ty), ir_store(dst, value))
}

/// `dst = move src` — cópia de uma variável com outro nome
pub fn ir_move(dst: &str, src: &str) -> String {
    format!("{dst} = move {src}")
}

/// `::Lxxxx::`
pub fn ir_label(name: &str) -> String {
    format!("::{name}::")
}

/// `br L`
pub fn ir_jmp(name: &str) -> String {
    format!("br {name}")
}

/// `brez cond, L` — salta se cond == 0
pub fn ir_jz(cond: &str, name: &str) -> String {
    format!("brez {cond}, {name}")
}

/// `brnz cond, L` — salta se cond != 0
pub fn ir_jnz(cond: &str, name: &str) -> String {
    format!("brnz {cond}, {name}")
}

/// Sem tipo na instrução. Sinal vai no nome do op (sdiv/udiv, slt/ult...),
/// o tamanho vem das alocações dos operandos.
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
