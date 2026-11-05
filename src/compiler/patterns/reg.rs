use crate::{
    compiler::{
        patterns::types::CarlaType,
        scanner::{
            BinOP::{IterConcat, IterConcatEq},
            TokenKind::{self, BinaryOperator, Comma, Hexa, Identifier, Integer},
        },
        symbols::Symbols,
    },
    morgana::MorganaType,
};

pub const IO_DEFAULT_BITS: u32 = 8;

pub fn foo(id: &str, slice: Option<Slice>, size: u32) -> bool {
    let _ = (id, slice, size);
    true
}

#[derive(Debug, Clone, PartialEq)]
pub enum RegKind {
    Named(String),
    Io(u64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegRef {
    pub kind: RegKind,
    pub bits: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Slice {
    pub lo: u32,
    pub width: u32,
}

impl RegRef {
    pub fn is_io(&self) -> bool {
        matches!(self.kind, RegKind::Io(_))
    }

    fn operands(&self) -> String {
        match &self.kind {
            RegKind::Named(n) => format!("{n}, {}", self.bits),
            RegKind::Io(a) => format!("{a:#x}, {}", self.bits),
        }
    }

    fn family(&self) -> &'static str {
        if self.is_io() {
            "io"
        } else {
            "reg"
        }
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

#[derive(Debug, Clone, PartialEq)]
pub enum Head {
    Name(String),
    Addr(u64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Raw {
    pub head: Head,
    pub bits: Option<u32>,
    pub range: Option<(u32, u32, bool)>,
}

const SYNTAX: &str = "expected `id, size`, `0xADDR`, `0xADDR, size`, or either of them followed by `, lo..hi` (e.g. `ax, 64`, `0x25`, `ax, 64, 0..3`)";

fn to_u32(n: i128, what: &str) -> Result<u32, String> {
    u32::try_from(n)
        .map_err(|_| format!("{what} must be a non-negative number that fits in 32 bits, found {n}"))
}

pub fn parse_args(toks: &[&TokenKind]) -> Result<Raw, String> {
    let segments: Vec<&[&TokenKind]> = toks.split(|k| matches!(**k, Comma)).collect();

    let head = match segments[0] {
        [Identifier(name)] => Head::Name(name.clone()),
        [Hexa(addr)] => Head::Addr(
            u64::try_from(*addr).map_err(|_| format!("invalid mapped io address: {addr:#x}"))?,
        ),
        [] => return Err("it is empty: ".to_string() + SYNTAX),
        _ => return Err("the first argument must be an id (`ax`) or a hexadecimal address (`0x25`)".into()),
    };

    let mut bits = None;
    let mut range = None;

    for seg in &segments[1..] {
        match seg {
            [Integer(n)] => {
                if range.is_some() {
                    return Err("the size must come before the slice (`ax, 64, 0..3`)".into());
                }
                if bits.is_some() {
                    return Err("the size was given twice".into());
                }
                bits = Some(to_u32(*n, "the size")?);
            }
            [Integer(lo), BinaryOperator(op), Integer(hi)]
                if matches!(op, IterConcat | IterConcatEq) =>
            {
                if range.is_some() {
                    return Err("only one slice (`lo..hi`) is allowed".into());
                }
                range = Some((
                    to_u32(*lo, "the first bit of a slice")?,
                    to_u32(*hi, "the last bit of a slice")?,
                    matches!(op, IterConcatEq),
                ));
            }
            _ => return Err(SYNTAX.into()),
        }
    }

    if matches!(head, Head::Name(_)) && bits.is_none() {
        return Err("an id needs its size in bits as the second argument (e.g. `ax, 64`)".into());
    }

    Ok(Raw { head, bits, range })
}

pub fn validate(raw: &Raw) -> Result<(RegRef, Option<Slice>), String> {
    let bits = match &raw.head {
        Head::Name(_) => raw.bits.expect("parse_args guarantees the size of an id"),
        Head::Addr(_) => raw.bits.unwrap_or(IO_DEFAULT_BITS),
    };

    let slice = match raw.range {
        None => None,
        Some((lo, hi, inclusive)) => {
            let end = if inclusive { hi as u64 + 1 } else { hi as u64 };
            if end <= lo as u64 {
                return Err(format!("the slice `{lo}..{}{hi}` has no bits", if inclusive { "=" } else { "" }));
            }
            Some(Slice { lo, width: (end - lo as u64) as u32 })
        }
    };

    let kind = match &raw.head {
        Head::Name(id) => {
            if !foo(id, slice, bits) {
                return Err(format!("`{id}` with {bits} bits{} isn't acceptable", match slice {
                    Some(s) => format!(" and the slice {}..{}", s.lo, s.lo + s.width),
                    None => String::new(),
                }));
            }
            RegKind::Named(id.clone())
        }
        Head::Addr(a) => RegKind::Io(*a),
    };

    Ok((RegRef { kind, bits }, slice))
}

pub fn int_type(syms: &Symbols, bits: u32, signed: bool) -> Result<CarlaType, String> {
    let want = if signed {
        MorganaType::Integer(bits as _)
    } else {
        MorganaType::UInteger(bits as _)
    };

    let (long, short) = if signed { ("int", "i") } else { ("uint", "u") };
    let mut names = vec![
        format!("{long}{bits}"),
        format!("{short}{bits}"),
        long.to_string(),
        short.to_string(),
    ];
    if bits == 8 && !signed {
        names.push("byte".to_string());
    }

    for name in &names {
        if let Some(ty) = CarlaType::named(name, syms) {
            if ty.to_morgana() == want {
                return Ok(ty);
            }
        }
    }

    Err(format!(
        "no {} {bits}-bit integer type is registered in Symbols (looked for {}); add its name in `reg::int_type`",
        if signed { "signed" } else { "unsigned" },
        names.iter().map(|n| format!("`{n}`")).collect::<Vec<_>>().join(", ")
    ))
}
