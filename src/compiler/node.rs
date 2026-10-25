use crate::compiler::patterns::{
    conditional::Conditional, declaration::Variable, expressions::{Compiled, Expr}, r#extern::Extern, for_loop::ForLoop, incdec::IncDec, lambda::Lambda, types::CarlaType, unfold::Unfold, while_loop::WhileLoop,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    InternalType(CarlaType),
    Declaration(Variable),
    Expression(Compiled, bool),
    Extension(CarlaType, Vec<Node>),
    STMTDefer(Compiled),
    STMTReturn(Compiled),
    Syscall(Expr, i128),
    Function(Lambda),
    Declare(Extern),
    Unfoldable(Unfold),
    Conditional(Conditional),
    While(WhileLoop),
    For(ForLoop),
    IncDec(IncDec),
    Break,
    Continue,
    Nop,
}
