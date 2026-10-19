use crate::compiler::patterns::{
    declaration::Variable, expressions::Expr, r#extern::Extern, lambda::Lambda, types::CarlaType,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    InternalType(CarlaType),
    Declaration(Variable),
    Expression(Expr, bool),
    Function(Lambda),
    Declare(Extern),
    Nop,
}
