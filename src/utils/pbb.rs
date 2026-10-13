use crate::compiler::scanner::PBB;

pub fn become_block(pbb: &PBB) -> &str {
    match pbb {
        PBB::Bracket => "[]",
        PBB::Brace => "{}",
        PBB::Paren => "()",
    }
}
