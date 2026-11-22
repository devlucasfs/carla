use std::collections::HashMap;
use crate::compiler::{
    codegen::Codegen, patterns::{expressions::ExprId, reg::{Access, Loc, Slice, Space}, types::CarlaType}, symbols::Symbols,
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
            MorganaType::Integer(bits) if *bits != 0 => format!("i{bits}"),
            MorganaType::UInteger(bits) if *bits != 0 => format!("u{bits}"),

            MorganaType::Integer(_) => format!("int"),
            MorganaType::UInteger(_) => format!("uint"),

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
    pub registers: HashMap<String, (Loc, Option<Slice>)>,
    pub register_steps: HashMap<ExprId, (Loc, Option<Slice>, usize)>,
    pub result_register: Option<(Loc, Option<Slice>, usize)>,
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
            registers: HashMap::new(),
            register_steps: HashMap::new(),
            result_register: None,
        }
    }
}

impl Loc {
    pub fn is_whole_lane(&self, s: &Slice) -> bool {
        matches!(&self.lane, Some(l) if s.width == l.ty.bits)
    }

    pub fn is_reg(&self) -> bool {
        matches!(self.space, Space::Reg(_))
    }

    pub fn is_sreg(&self) -> bool {
        matches!(self.space, Space::Sreg(_))
    }

    pub fn is_vreg(&self) -> bool {
        matches!(self.space, Space::Vreg(_))
    }

    pub fn is_io(&self) -> bool {
        matches!(self.space, Space::Io(_))
    }

    pub fn is_mem(&self) -> bool {
        matches!(self.space, Space::Mem(_))
    }

    fn operands(&self) -> String {
        match &self.space {
            Space::Reg(n) | Space::Sreg(n) | Space::Vreg(n) => format!("{n}, {}", self.bits),
            Space::Io(a) | Space::Mem(a) => format!("{a:#x}, {}", self.bits),
        }
    }

    fn family(&self) -> &'static str {
        match self.space {
            Space::Reg(_) => "reg",
            Space::Sreg(_) => "sreg",
            Space::Vreg(_) => "vreg",
            Space::Io(_) => "io",
            Space::Mem(_) => "mem",
        }
    }

    /// Reads (`load`, `get`) are refused on a `wo` location.
    pub fn check_load(&self) -> Result<(), String> {
        match self.access {
            Access::Wo => Err("this location is write-only (`wo`): it can't be read".into()),
            _ => Ok(()),
        }
    }

    /// Writes (`store`) are refused on a `ro` location.
    pub fn check_store(&self) -> Result<(), String> {
        match self.access {
            Access::Ro => Err("this location is read-only (`ro`): it can't be written".into()),
            _ => Ok(()),
        }
    }

    /// `set` reads the old value, replaces the bits and writes it back, so it needs both.
    pub fn check_modify(&self) -> Result<(), String> {
        self.check_load()?;
        self.check_store()
    }

    pub fn ir_load(&self, dest: &str) -> String {
        format!("{dest} = {}_load {}", self.family(), self.operands())
    }

    pub fn ir_store(&self, value: &str) -> String {
        format!("{}_store {}, {value}", self.family(), self.operands())
    }

    pub fn ir_get(&self, dest: &str, s: &Slice) -> String {
        format!("{dest} = {}_get {}, {}, {}", self.family(), self.operands(), s.lo, s.width)
    }

    pub fn ir_set(&self, s: &Slice, value: &str) -> String {
        format!("{}_set {}, {}, {}, {value}", self.family(), self.operands(), s.lo, s.width)
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

pub fn ir_addr_of(dst: &str, src: &str) -> String {
    format!("{dst} = ptr_to {src}")
}

pub fn ir_deref(dst: &str, ty: &MorganaType, ptr: &str) -> String {
    format!("{dst} = deref {} {ptr}", ty.to_string())
}

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

    lines.push(ir_const(&base, &MorganaType::Ptr, &addr));

    for ((name, ty), offset) in names.iter().zip(types).zip(offsets) {
        let Some(name) = name else { continue };

        let cur = format!("tc{}", id());
        lines.push(format!("{cur} = load {base}"));

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
