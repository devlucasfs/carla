#[derive(Debug, Clone, PartialEq)]
pub enum MorganaType {
    UInteger(i32),
    Integer(i32),
    Void,
    Ptr,
}
