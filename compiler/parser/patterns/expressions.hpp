// #pragma once
//
// #include "../pattern.hpp"
// #include "compiler_outputs.hpp"
// #include "parser/ctx.hpp"
// #include "parser/nodes/static.hpp"
// #include "tokenizer/token.hpp"
// #include "tokenizer/token_kind.hpp"
// #include <cstddef>
// #include <cstdint>
// #include <cstdlib>
// #include <iostream>
// #include <optional>
// #include <string>
// #include <variant>
//
// #define CTRUE  = int8_t(255);
// #define CFALSE = int8_t(0);
//
// #define CARLA_PATTERN_ARGUMENTS_STATIC_EXPR carla::comptime_value *comptime, Symt *sym, size_t *index, const std::vector<pContext> *ctx
// #define CARLA_PATTERN_ARGUMENTS_EXPORT_STATIC_EXPR comptime, sym, index, ctx
// #define CARLA_PATTERN_ARGUMENTS_EXPORT_STATIC_EXPR_CIV(c, i, v) c, sym, i, v
// #define CARLA_PATTERN_ARGUMENTS_EXPORT_STATIC_EXPR_C(c) c, sym, index, ctx
//
// void trycall(); // todo!
//
// struct ExpressionRules {
//     static bool strings(CARLA_PATTERN_ARGUMENTS_STATIC_EXPR) {
//         auto backup = *index;
//         std::stringstream ss;
//         while(true) {
//             auto data = ctx->at(*index);
//
//             switch(data.kind) {
//                 case Common: {
//                     auto tk = std::get<Token>(data.content);
//                     switch(tk.kind) {
//                         case INTEGER:
//                         case _FLOAT: ss << tk.lexeme;
//                         break;
//
//                         case NIL:
//                         case _TRUE:
//                         case _FALSE: ss << kindKeywordToString(tk.kind);
//                         break;
//
//                         case STRING: ss << tk.lexeme.substr(1, tk.lexeme.size()-2);
//                         break;
//
//                         default: {
//                             // trycall();
//                             CompilerOutputs::Fatal("You can't use this value in a static string.");
//                         } break;
//                     }
//                 } break;
//                 case Block: break; // todo!
//             }
//
//             if( (*index) + 1 >= ctx->size() ) break;
//
//             auto concat = ctx->at(++(*index));
//             if( concat.kind != Common ) CompilerOutputs::Fatal("You can't to use this operator in a static string expression.");
//             if( std::get<Token>(concat.content).kind != ITER_CONCAT ) CompilerOutputs::Fatal("You can't to use this operator in a static string expression.");
//         }
//
//         std::string str = ss.str();
//         bool empty = str.empty();
//
//         if( empty ) *index = backup;
//         else *comptime = str;
//         return !empty;
//     }
// };
//
// void static_expressions(CARLA_PATTERN_ARGUMENTS_STATIC_EXPR);
// bool unknown_expression(CARLA_PATTERN_ARGUMENTS) {
//     CARLA_PATTERN_STARTS(bool, false);
//     CARLA_PEEK_NEXT(first, _default);
//
//     if( first.kind == Block ) CARLA_RETURN_DEFAULT; // TODO! Normal Expressions
//
//     auto token = std::get<Token>(first.content);
//     switch(token.kind) {
//         case _CONSTEXPR: {
//             (*index)++;
//
//             CARLA_PEEK_NEXT(expr, _default);
//             if( expr.kind != Block ) CARLA_RETURN_DEFAULT;
//
//             auto block = std::get<std::vector<pContext>>(expr.content);
//             size_t i(0);
//             carla::comptime_value comptime = std::monostate();
//
//             static_expressions(&CARLA_PATTERN_ARGUMENTS_EXPORT_STATIC_EXPR_CIV(comptime, &i, &block));
//
//             if( std::holds_alternative<std::monostate>(comptime) ) CompilerOutputs::Fatal("Invalid constexpr.");
//             return true;
//         }
//
//         default: {} break; // TODO! Normal Expressions
//     }
//
//     return false;
// }
//
// carla::StaticValue getval(Token& content) {
//     switch(content.kind) {
//         case STRING: return carla::StaticValue( content.lexeme.substr(1, content.lexeme.size()-2) );
//         case INTEGER: return carla::StaticValue( std::atoll(content.lexeme.c_str()) );
//         case _FLOAT: return carla::StaticValue( std::atof(content.lexeme.c_str()) );
//         case IDENTIFIER: {} break;
//         default: break;
//     }
//
//     return carla::StaticValue(std::monostate());
// }
//
// std::pair<std::optional<std::string>, bool> transform_as_bool(carla::StaticValue val) {
//     if( std::holds_alternative<std::string>(val.data) )
//         return { std::optional<std::string>("Strings are not kinda boolean values."), false };
// }
//
// template<typename T>
// bool def(T* p, T& tk) {
//     *p = tk;
//     return true;
// }
//
// void static_expressions(CARLA_PATTERN_ARGUMENTS_STATIC_EXPR) {
//     /*
//      * if constexpr( TARGET_OS == "windows" ) {} -- agree
//      *
//      */
//
//     const int8_t
//         NORMAL = 0b00000000,
//         NOT_OP = 0b00000001,
//         REVERSE_OP = 0b00000010;
//
//     int8_t state = NORMAL;
//     auto first = ctx->at(*index);
//     if( first.kind == Common )
//         switch(std::get<Token>(first.content).kind) {
//         case BANG:  state |= NOT_OP;
//         case MINUS: state |= REVERSE_OP;
//         default: goto ignore_add;
//     };
//
//     (*index)++;
//     ignore_add: ;
//
//     if( ctx->size() <= (*index) ) CompilerOutputs::Fatal("Was expected an expression node, but was found " + tokenKindToString(CARLA_EOF));
//     auto left = ctx->at(*index);
//
//     Token t;
//     if( left.kind == Common || def(&t, std::get<Token>(left.content)) )
//     /* -> */ switch (t.kind) {
//         case STRING: {
//             std::cout << "entrou como uma string\n";
//             if(! ExpressionRules::strings(CARLA_PATTERN_ARGUMENTS_EXPORT_STATIC_EXPR) ) CompilerOutputs::Fatal("Invalid string comptime expression");
//             return;
//         } break;
//         default: {
//             std::cout << "não se sabe `" << tokenKindToString(t.kind) << "` with `" << t.lexeme << "`\n";
//         }
//     }
//
// }
