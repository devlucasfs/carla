use crate::compiler::patterns::{declaration::Variable, lambda::Lambda, types::CarlaType};

#[derive(Debug, PartialEq)]
pub enum Node {
    InternalType(CarlaType),
    Declaration(Variable),
    Function(Lambda),
    Nop,
}
