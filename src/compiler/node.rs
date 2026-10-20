use crate::compiler::patterns::{
    declaration::Variable, expressions::Compiled, r#extern::Extern, lambda::Lambda,
    types::CarlaType,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    InternalType(CarlaType),
    Declaration(Variable),
    Expression(Compiled, bool),
    STMTDefer(Compiled),
    Function(Lambda),
    Declare(Extern),
    Nop,
}
