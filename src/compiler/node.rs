use crate::compiler::patterns::{
    declaration::Variable, expressions::{Compiled, Expr}, r#extern::Extern, lambda::Lambda, types::CarlaType, unfold::Unfold,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    InternalType(CarlaType),
    Declaration(Variable),
    Expression(Compiled, bool),
    Extension(CarlaType, Vec<Node>),
    STMTDefer(Compiled),
    Syscall(Expr, i128),
    Function(Lambda),
    Declare(Extern),
    Unfoldable(Unfold),
    Nop,
}
