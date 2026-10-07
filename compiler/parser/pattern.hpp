// Faz o Handler dos possiveis Patterns

#pragma once

#include "ctx.hpp"
#include "node.hpp"
#include "parser/nodes/then.hpp"
#include "symbols.hpp"
#include "tokenizer/scanner.hpp"
#include "tokenizer/token.hpp"
#include <algorithm>
#include <climits>
#include <filesystem>
#include <iostream>
#include <stack>
#include <string>
#include <vector>

#define CARLA_THEN_COMPATIBLE_NODE_KINDS \
    X(MORG_QUOTE)

static bool last_one_compatible_with_then = false;

#define CARLA_PATTERN_ARGUMENTS pNode *result, Symt *sym, size_t *index, const std::vector<pContext>* ctx
#define CARLA_PATTERN_EXPORT result, sym, index, ctx
#define CARLA_PATTERN_STARTS(return_t, data) size_t backup = *index; \
                                             return_t _default = data;

#define CARLA_RETURN_DEFAULT do { (*index) = backup; return _default; } while(0)

#define CARLA_GET_NEXT(id, val) if( *index >= ctx->size() ) return val; \
                                auto id = (*ctx)[(*index)++]

#define CARLA_INDEX_NEXT(id, val, ctx, index) if( *index >= ctx->size() ) return val; \
                                              auto id = (*ctx)[(*index)++]


#define CARLA_PEEK_NEXT(id, val) if( *index >= ctx->size() ) return val; \
                                 auto id = (*ctx)[*index]

#include "../utils/result.hpp"
#include "../compiler_outputs.hpp"
#include "../tokenizer/token_kind.hpp"

Result pattern(CARLA_PATTERN_ARGUMENTS, bool expr=false);

long current_line = 0;
long less_lines = 0;

std::stack<std::tuple<std::string, long>> file_stack;

void *special_fstack = NULL;
extern std::string absolute_main_file;

#include "parser/patterns/namespace.hpp"
#include "./patterns/declaration.hpp"
#include "./patterns/expressions.hpp"
#include "./patterns/statement.hpp"
#include "./patterns/lambda.hpp"
#include "./patterns/macros.hpp"

#include <cstddef>
#include <sstream>

std::string unknownPattern(const std::vector<pContext>* ctx, size_t *index);

Result pattern(CARLA_PATTERN_ARGUMENTS, bool expr) {
    const pContext& context = (*ctx)[*index];

    if( expr && context.kind == Block ) return Err{""};
    if( expr && context.kind == Common ) {
        Token tk = std::get<Token>(context.content);
        switch(tk.kind) {
        // case IDENTIFIER:
        // if( call(CARLA_PATTERN_EXPORT) ) return Some{};
        // else return Err{unknownPattern(ctx, index)};
        default: return Err{""};
        }
    }

    if( context.kind == Block ) {
        if( lambda(CARLA_PATTERN_EXPORT) ) return Some{};
        else if( declaration(CARLA_PATTERN_EXPORT) ) return Some{};
        else return Err{unknownPattern(ctx, index)};
    }

    Token tk = std::get<Token>(context.content);

    if( tk.kind == COLON && last_one_compatible_with_then ) {
        (*index)++;
        result->~pNode();
        new (result) pNode(carla::Then());
        return Some{};
    }

    current_line = tk.line - less_lines;

    switch(tk.kind) {
    case PUSH_F: {
        if( *index >= ctx->size() ) CompilerOutputs::Fatal("You can't push `@void`");

        auto data = (*ctx)[++(*index)];
        if( data.kind != Common ) CompilerOutputs::Fatal("You can't push a Block");

        auto token = std::get<Token>(data.content);
        if( token.kind != STRING ) CompilerOutputs::Fatal("You can't push a " + tokenKindToString(token.kind));
        if( tk.line != token.line ) CompilerOutputs::Fatal("The content to be pushed need to be in the same line.");

        file_stack.push({ token.lexeme, tk.line });

        (*index)++;
        result->~pNode();
        new (result) pNode(carla::Nop());
        return Some{};
    } break;

    case POP_F: {
        auto [_, line] = file_stack.top();
        less_lines += tk.line - line + 1;
        file_stack.pop();

        (*index)++;
        result->~pNode();
        new (result) pNode(carla::Nop());
        return Some{};
    } break;

    case SEMICOLON: {
        (*index)++;
        result->~pNode();
        new(result) pNode(carla::Nop());
        return Some{};
    };

    case _CONSTEXPR:
    if( unknown_expression(CARLA_PATTERN_EXPORT) ) return Some{};
    else return Err{unknownPattern(ctx, index)};
    case START:
    if( macros(CARLA_PATTERN_EXPORT, tk.kind) ) return Some{};
    else return Err{unknownPattern(ctx, index)};
    case PUTS:
    if( statement(CARLA_PATTERN_EXPORT, "puts") ) return Some{};
    else return Err{unknownPattern(ctx, index)};
    case _NAMESPACE:
    if( _namespace(CARLA_PATTERN_EXPORT) ) return Some{};
    else return Err{unknownPattern(ctx, index)};
    case IDENTIFIER:
    // if( call(CARLA_PATTERN_EXPORT) ) return Some{};
    if( declaration(CARLA_PATTERN_EXPORT) ) return Some{};
    else return Err{unknownPattern(ctx, index)};
    // case INTEGER:
    // case _FLOAT:
    // case STRING:
    // if( expression(CARLA_PATTERN_EXPORT) ) return Some{};
    // else return Err{unknownPattern(ctx, index)};
    default: return Err{unknownPattern(ctx, index)};
    }

    return Err{unknownPattern(ctx, index)};
}

std::string unknownPattern(const std::vector<pContext>* ctx, size_t *index) {
    std::stringstream str;
    std::stringstream buff;
    std::stringstream line;
    pContext context;

    if( *index >= ctx->size() ) {
        buff << "Token Data not found";
        line << Colorizer::BOLD_YELLOW << "Carla[Internal<Line(?:Numeric!)>]" << Colorizer::RESET;
        goto __print_err;
    }

    context = (*ctx)[*index];
    if( context.kind == Common ) {
        Token tk = std::get<Token>(context.content);
        buff << ((tk.lexeme.length() == 0) ? tokenKindToString(tk.kind) : tk.lexeme);
        line << "file:line // TODO! Implement a parser to read the line of the file based in the precompiler output";
    } else {
        buff << Colorizer::BOLD_YELLOW << "Carla[Internal<Block>]" << Colorizer::RESET;
        line << Colorizer::BOLD_YELLOW << "Carla[Internal<Line(?:Numeric!)>]" << Colorizer::RESET;
    }

    __print_err:
    str << Colorizer::RED << "Unknown pattern at context index " << *index << " (addr. " << Colorizer::GREEN << index << Colorizer::RED << ')' << Colorizer::RESET << ": '" << buff.str() << "'\n";
    str << Colorizer::DARK_GREY << "└─ " << Colorizer::RESET << "Expected another pattern in " << line.str() << "\n";
    return str.str();
}
