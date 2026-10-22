use crate::compiler::{patterns::types::CarlaType, symbols::Symbols};

pub const EXTENSION_PREFIX: &str = "__carla_abi_extension";

impl CarlaType {
    /// `Foo[4]**` -> `Foo_A4_PP`, `|int, int|` -> `T_int_int_E`, `|_|*` -> `Wild_P`
    pub fn abi_name(&self) -> String {
        let mut out = if !self.tuple.is_empty() {
            let inner: Vec<String> = self.tuple.iter().map(|t| t.abi_name()).collect();
            format!("T_{}_E", inner.join("_"))
        } else if let Some(base) = &self.base {
            base.id.clone()
        } else {
            "Wild".to_string()
        };

        for d in &self.dim {
            out.push_str(&format!("_A{d}"));
        }
        for _ in 0..self.levels {
            out.push_str("_P");
        }
        out
    }
}

pub fn extension_symbol(ty: &CarlaType, id: &str) -> String {
    format!("{EXTENSION_PREFIX}_{}_{id}", ty.abi_name())
}

pub fn sym_name_by_abi(syms: &Symbols, id: &str) -> String {
    match syms.extending() {
        Some(ty) => extension_symbol(ty, id),
        None => id.to_string(),
    }
}
