use crate::{
    compiler::symbols::{Symbol::BType, Symbols},
    morgana::MorganaType::{Integer, Ptr, UInteger, Void},
};

pub fn append(symbols: &mut Symbols) {
    symbols.push("char", BType(UInteger(8)));
    symbols.push("bool", BType(Integer(8)));
    symbols.push("byte", BType(Integer(8)));
    symbols.push("void", BType(Void));
    symbols.push("generic_ptr", BType(Ptr));

    // Todo! Size needs to be decided by the target arch
    symbols.push("int", BType(Integer(64)));
    symbols.push("uint", BType(UInteger(64)));

    symbols.push("word", BType(Integer(32)));
    symbols.push("iword", BType(Integer(32)));
    symbols.push("uword", BType(UInteger(32)));

    symbols.push("int8", BType(Integer(8)));
    symbols.push("int16", BType(Integer(16)));
    symbols.push("int32", BType(Integer(32)));
    symbols.push("int64", BType(Integer(64)));

    symbols.push("uint8", BType(UInteger(8)));
    symbols.push("uint16", BType(UInteger(16)));
    symbols.push("uint32", BType(UInteger(32)));
    symbols.push("uint64", BType(UInteger(64)));
}
