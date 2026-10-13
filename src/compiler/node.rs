use crate::compiler::patterns::{
    declaration::Variable, r#extern::Extern, lambda::Lambda, types::CarlaType,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    InternalType(CarlaType),
    Declaration(Variable),
    Function(Lambda),
    Declare(Extern),
    Nop,
}
