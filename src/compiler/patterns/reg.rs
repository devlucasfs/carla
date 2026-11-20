//! Locations: everything that is not a plain variable (registers, system registers, SIMD
//! registers, memory-mapped addresses and io ports).
//!
//! The programmer ALWAYS says what kind of location it is; the compiler never guesses.
//!
//! ```text
//! reg,  ax, 64                    general register, size required
//! sreg, ccount, 32                special / system register (msr, csr, cr0, sr, ...)
//! vreg, xmm0, 128                 SIMD register, size required
//! vreg, ymm0, 256, u32, 5         lane 5 of ymm0 seen as 8 lanes of u32
//! vreg, xmm1, 128, u8, 3, 2..6    bits 2..6 INSIDE lane 3
//! mem,  0x2000                    memory-mapped address, 8 bits by default
//! mem,  0x2000, 32, ro            32 bits, read only
//! io,   0x25, 8, 0..=3            io port, bits 0..=3
//! ```
//!
//! Order of the arguments after the target: `size`, then `lane type` + `lane index`
//! (vreg only), any access mode (`ro` / `wo` / `rw`, default `rw`), and finally the slice
//! `lo..hi` / `lo..=hi`. A slice on a lane is relative to the lane; the compiler turns it
//! into an absolute bit range, so the IR never needs to know about lanes.

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

pub const DEFAULT_BITS: u32 = 8;
const MAX_REG_BITS: u32 = 128;
const MAX_VEC_BITS: u32 = 2048;


#[derive(Debug, Clone, PartialEq)]
pub enum Space {
    Reg(String),
    Sreg(String),
    Vreg(String),
    Mem(u64),
    Io(u64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Access {
    Rw,
    Ro,
    Wo,
}

impl Access {
    fn parse(s: &str) -> Option<Access> {
        match s {
            "rw" => Some(Access::Rw),
            "ro" => Some(Access::Ro),
            "wo" => Some(Access::Wo),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LaneKind {
    Int { signed: bool },
    Float,
}

/// Element type of a SIMD lane: `u8 u16 u32 u64 u128`, `i8 .. i128`, `f16 f32 f64`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LaneTy {
    pub bits: u32,
    pub kind: LaneKind,
}

impl LaneTy {
    fn parse(s: &str) -> Option<LaneTy> {
        let kind = match s.chars().next()? {
            'u' => LaneKind::Int { signed: false },
            'i' => LaneKind::Int { signed: true },
            'f' => LaneKind::Float,
            _ => return None,
        };

        let bits: u32 = s[1..].parse().ok()?;
        let ok = match kind {
            LaneKind::Int { .. } => matches!(bits, 8 | 16 | 32 | 64 | 128),
            LaneKind::Float => matches!(bits, 16 | 32 | 64),
        };

        ok.then_some(LaneTy { bits, kind })
    }

    pub fn name(&self) -> String {
        let c = match self.kind {
            LaneKind::Int { signed: false } => 'u',
            LaneKind::Int { signed: true } => 'i',
            LaneKind::Float => 'f',
        };
        format!("{c}{}", self.bits)
    }

    pub fn value_type(&self, syms: &Symbols) -> Result<CarlaType, String> {
        match self.kind {
            LaneKind::Int { signed } => int_type(syms, self.bits, signed),
            LaneKind::Float => int_type(syms, self.bits, false),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lane {
    pub ty: LaneTy,
    pub index: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Loc {
    pub space: Space,
    pub bits: u32,
    pub lane: Option<Lane>,
    pub access: Access,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Slice {
    pub lo: u32,
    pub width: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Raw {
    pub space: Space,
    pub bits: Option<u32>,
    pub elem: Option<LaneTy>,
    pub lane: Option<u32>,
    pub access: Option<Access>,
    pub range: Option<(u32, u32, bool)>,
}

const SYNTAX: &str = "expected `KIND, target[, size][, lane type, lane index][, ro|wo|rw][, lo..hi]` where KIND is `reg`, `sreg`, `vreg`, `mem` or `io` (e.g. `reg, ax, 64`, `io, 0x25`, `mem, 0x2000, 32, ro`, `vreg, ymm0, 256, u32, 5`)";

fn to_u32(n: i128, what: &str) -> Result<u32, String> {
    u32::try_from(n)
        .map_err(|_| format!("{what} must be a non-negative number that fits in 32 bits, found {n}"))
}

fn to_name(target: &[&TokenKind], kind: &str) -> Result<String, String> {
    match target {
        [Identifier(n)] => Ok(n.clone()),
        _ => Err(format!("`{kind}` expects a register name (e.g. `{kind}, ax, 64`)")),
    }
}

fn to_addr(target: &[&TokenKind], kind: &str) -> Result<u64, String> {
    match target {
        [Hexa(a)] => u64::try_from(*a).map_err(|_| format!("invalid {kind} address: {a:#x}")),
        [Integer(a)] => u64::try_from(*a).map_err(|_| format!("invalid {kind} address: {a}")),
        _ => Err(format!("`{kind}` expects an address (e.g. `{kind}, 0x25`)")),
    }
}

pub fn parse_args(toks: &[&TokenKind]) -> Result<Raw, String> {
    let segments: Vec<&[&TokenKind]> = toks.split(|k| matches!(**k, Comma)).collect();

    let kind = match segments[0] {
        [Identifier(k)] => k.as_str(),
        [] => return Err("it is empty: ".to_string() + SYNTAX),
        _ => {
            return Err(format!(
                "the first argument must be the location kind: `reg`, `sreg`, `vreg`, `mem` or `io`. {SYNTAX}"
            ))
        }
    };

    let target: &[&TokenKind] = segments.get(1).copied().unwrap_or(&[]);

    let space = match kind {
        "reg" => Space::Reg(to_name(target, kind)?),
        "sreg" => Space::Sreg(to_name(target, kind)?),
        "vreg" => Space::Vreg(to_name(target, kind)?),
        "mem" => Space::Mem(to_addr(target, kind)?),
        "io" => Space::Io(to_addr(target, kind)?),
        other => {
            return Err(format!(
                "unknown location kind `{other}`: use `reg`, `sreg`, `vreg`, `mem` or `io`"
            ))
        }
    };

    let mut bits = None;
    let mut elem: Option<LaneTy> = None;
    let mut lane = None;
    let mut access = None;
    let mut range = None;

    for seg in segments.iter().skip(2) {
        match seg {
            [Integer(n)] => {
                if range.is_some() {
                    return Err("the size and the lane must come before the slice (`vreg, xmm0, 128, u32, 2, 0..8`)".into());
                }
                if bits.is_none() {
                    bits = Some(to_u32(*n, "the size")?);
                } else if elem.is_some() && lane.is_none() {
                    lane = Some(to_u32(*n, "the lane index")?);
                } else if elem.is_some() {
                    return Err("the lane index was given twice".into());
                } else {
                    return Err("the size was given twice".into());
                }
            }

            [Identifier(w)] => {
                if range.is_some() {
                    return Err(format!("`{w}` must come before the slice (`lo..hi`)"));
                }

                if let Some(a) = Access::parse(w) {
                    if access.is_some() {
                        return Err("the access mode (`ro`, `wo`, `rw`) was given twice".into());
                    }
                    access = Some(a);
                } else if let Some(t) = LaneTy::parse(w) {
                    if !matches!(space, Space::Vreg(_)) {
                        return Err(format!("a lane type (`{w}`) only makes sense on a `vreg`"));
                    }
                    if elem.is_some() {
                        return Err("the lane type was given twice".into());
                    }
                    if bits.is_none() {
                        return Err("the size of the vector must come before the lane type (`vreg, xmm0, 128, u32, 2`)".into());
                    }
                    elem = Some(t);
                } else {
                    return Err(format!(
                        "unknown modifier `{w}`: expected `ro`, `wo`, `rw` or a lane type (`u8`..`u128`, `i8`..`i128`, `f16`, `f32`, `f64`)"
                    ));
                }
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

    if matches!(space, Space::Reg(_) | Space::Sreg(_) | Space::Vreg(_)) && bits.is_none() {
        return Err(format!("`{kind}` needs its size in bits as the third argument (e.g. `{kind}, {}, 64`)", match &space {
            Space::Reg(n) | Space::Sreg(n) | Space::Vreg(n) => n.as_str(),
            _ => "x",
        }));
    }

    if elem.is_some() && lane.is_none() {
        return Err("a lane type needs the lane index right after it (e.g. `vreg, xmm0, 128, u32, 2`)".into());
    }

    Ok(Raw { space, bits, elem, lane, access, range })
}

pub fn validate(raw: &Raw) -> Result<(Loc, Option<Slice>), String> {
    let is_vec = matches!(raw.space, Space::Vreg(_));
    let is_bus = matches!(raw.space, Space::Mem(_) | Space::Io(_));

    let bits = match &raw.space {
        Space::Reg(_) | Space::Sreg(_) | Space::Vreg(_) => {
            raw.bits.expect("parse_args guarantees the size of a register")
        }
        Space::Mem(_) | Space::Io(_) => raw.bits.unwrap_or(DEFAULT_BITS),
    };

    let max = if is_vec { MAX_VEC_BITS } else { MAX_REG_BITS };
    if bits == 0 || bits > max {
        return Err(format!("the size must be between 1 and {max} bits, found {bits}"));
    }
    if (is_bus || is_vec) && bits % 8 != 0 {
        return Err(format!("the size must be a multiple of 8 bits here, found {bits}"));
    }

    let lane = match (raw.elem, raw.lane) {
        (Some(ty), Some(index)) => {
            if bits % ty.bits != 0 {
                return Err(format!("a {bits}-bit vector can't be split in lanes of {}", ty.name()));
            }
            let count = bits / ty.bits;
            if index >= count {
                return Err(format!(
                    "lane {index} is out of range: a {bits}-bit vector has {count} lanes of {} (0..{count})",
                    ty.name()
                ));
            }
            Some(Lane { ty, index })
        }
        _ => None,
    };

    let (base, window) = match lane {
        Some(l) => (l.index * l.ty.bits, l.ty.bits),
        None => (0, bits),
    };

    let rel = match raw.range {
        None => None,
        Some((lo, hi, inclusive)) => {
            let end = if inclusive { hi as u64 + 1 } else { hi as u64 };
            if end <= lo as u64 {
                return Err(format!(
                    "the slice `{lo}..{}{hi}` has no bits",
                    if inclusive { "=" } else { "" }
                ));
            }
            if end > window as u64 {
                return Err(format!(
                    "the slice {lo}..{end} doesn't fit in the {window} bits of the {}",
                    if lane.is_some() { "lane" } else { "location" }
                ));
            }
            Some(Slice { lo, width: (end - lo as u64) as u32 })
        }
    };

    let slice = match (rel, lane) {
        (Some(s), _) => Some(Slice { lo: base + s.lo, width: s.width }),
        (None, Some(l)) => Some(Slice { lo: base, width: l.ty.bits }),
        (None, None) => None,
    };

    let moved = slice.map(|s| s.width).unwrap_or(bits);
    if moved > MAX_REG_BITS {
        return Err(format!(
            "this moves {moved} bits at once and the biggest value is {MAX_REG_BITS} bits: use a lane or a slice"
        ));
    }

    Ok((
        Loc { space: raw.space.clone(), bits, lane, access: raw.access.unwrap_or(Access::Rw) },
        slice,
    ))
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
        "no {} {bits}-bit integer type is registered in Symbols (looked for {}); add its name in `loc::int_type`",
        if signed { "signed" } else { "unsigned" },
        names.iter().map(|n| format!("`{n}`")).collect::<Vec<_>>().join(", ")
    ))
}
