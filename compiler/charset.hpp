#pragma once

#include "./parser/symbols.hpp"
#include "libs/morgana.hpp"
#include "libs/morgana/types.hpp"
#include <memory>

#define CARLA_CHARSET_EXPORTS_FIELDS \
    X(utf8, "ostr")

#define X1(id)      Y(id, #id)
#define X2(id, str) Y(id, str)
#define GET_MACRO(_1,_2,NAME,...) NAME
#define X(...) GET_MACRO(__VA_ARGS__, X2, X1)(__VA_ARGS__)
#define Y(x, y)                           \
constexpr auto x = y;                     \
constexpr auto const_##x = "const " y;    \
std::string d_##x = std::string(x);       \
std::string d_const_##x = std::string(x);
namespace carla::charset { CARLA_CHARSET_EXPORTS_FIELDS }
#undef GET_MACRO
#undef X2
#undef X1
#undef X
#undef Y

void charset(Symt& sym) {
    auto i0  = morgana::integer(0),
         i8  = morgana::integer(8),
         i16 = morgana::integer(16),
         i32 = morgana::integer(32),
         i64 = morgana::integer(64),

         /* Unsigned integer types */
         u0  = morgana::integer(0,  false),
         u8  = morgana::integer(8,  false),
         u16 = morgana::integer(16, false),
         u32 = morgana::integer(32, false),
         u64 = morgana::integer(64, false);

    auto ptr = morgana::ptr();
    auto void_t = morgana::void_t();
    auto oschar = morgana::ascii();

    sym.entry();
    sym.addSymbol(carla::charset::utf8, morgana::type(ptr));
    sym.addSymbol("char", morgana::type(oschar));

    sym.addSymbol("int", morgana::type(i0));
    sym.addSymbol("int8", morgana::type(i8));
    sym.addSymbol("int16", morgana::type(i16));
    sym.addSymbol("int32", morgana::type(i32));
    sym.addSymbol("int64", morgana::type(i64));

    sym.addSymbol("uint", morgana::type(u0));
    sym.addSymbol("uint8", morgana::type(i8));
    sym.addSymbol("uint16", morgana::type(i16));
    sym.addSymbol("uint32", morgana::type(i32));
    sym.addSymbol("uint64", morgana::type(i64));

    sym.addSymbol("void", morgana::type(void_t));
}
