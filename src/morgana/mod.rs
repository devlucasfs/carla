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

pub fn function(
    ctx: &mut Codegen,
    ret: MorganaType,
    identifier: String,
    args: Vec<(String, MorganaType)>,
    body: String,
) {
    ctx.append(format!(
        "{} {identifier}({}) {{\n{}{body}}}",
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
                    .join(" "),
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
