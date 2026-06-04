#pragma once

#include "../parser/nodes/expression.hpp"
#include "../parser/symbols.hpp"
#include "../parser/pattern.hpp"
#include <cstddef>
#include <iostream>
#include <stdexcept>
#include <string>
#include <variant>
#include <vector>

#define CAST_MACRO_STATIC_ARGUMENTS morgana::type original, morgana::type to, carla::InterpreterResult value
#define CAST_MACRO_DYNAMIC_ARGUMENTS Symbol *original, Symbol *to, Carla::Expr value

carla::InterpreterResult macro_cast_static(CAST_MACRO_STATIC_ARGUMENTS);

carla::InterpreterResult
assembler_cast(CARLA_PATTERN_ARGUMENTS, std::vector<carla::Expr> exprs)
{
    std::string first_error = "The first argument must be a radical type";
    std::string second_error = "The second argument must be a radical type";

    auto first = exprs.at(0);
    auto first_ast = first.ast;
    if( first.ast.kind != carla::ExprContext::Value ) throw std::runtime_error(first_error);

    auto first_ctx = std::get<pContext>(first_ast.content);
    if( first_ctx.kind != Common ) throw std::runtime_error(first_error);

    auto first_tk = std::get<Token>(first_ctx.content);
    if( first_tk.kind != IDENTIFIER ) throw std::runtime_error(first_error);

    auto first_lex = first_tk.lexeme;
    auto first_sym = sym->findSymbol(first_lex);
    if( first_sym == nullptr ) throw std::runtime_error(first_error);

    if(! std::holds_alternative<morgana::type>(*first_sym) ) throw std::runtime_error(first_error);

    auto first_arg = std::get<morgana::type>(*first_sym);

    auto second = exprs.at(1);
    auto second_ast = second.ast;
    if( second.ast.kind != carla::ExprContext::Value ) throw std::runtime_error(second_error);

    auto second_ctx = std::get<pContext>(second_ast.content);
    if( second_ctx.kind != Common ) throw std::runtime_error(second_error);

    auto second_tk = std::get<Token>(second_ctx.content);
    if( second_tk.kind != IDENTIFIER ) throw std::runtime_error(second_error);

    auto second_lex = second_tk.lexeme;
    auto second_sym = sym->findSymbol(second_lex);
    if( second_sym == nullptr ) throw std::runtime_error(second_error);

    if(! std::holds_alternative<morgana::type>(*second_sym) ) throw std::runtime_error(second_error);

    auto second_arg = std::get<morgana::type>(*second_sym);

    std::string third_error = "The third argument must be a value";
    auto third = exprs.at(2);

    if( third.is_static ) {
        return macro_cast_static(first_arg, second_arg, third.data);
    }

    throw std::runtime_error("not implemented yet");
}

carla::InterpreterResult
macro_cast_static(CAST_MACRO_STATIC_ARGUMENTS)
{
    carla::InterpreterResult r;

    if( std::holds_alternative<morgana::integer>(to) ) r = size_t(0);
    if( std::holds_alternative<morgana::ptr>(to) ) r = "";

    if( std::holds_alternative<size_t>(value) && std::holds_alternative<morgana::integer>(original) ) {
        auto val = std::get<size_t>(value);
        if( std::holds_alternative<morgana::ptr>(to) ) return std::to_string(val);
        else return val;
    }

    if( std::holds_alternative<std::string>(value) && std::holds_alternative<morgana::ptr>(original) ) {
        auto val = std::get<std::string>(value);
        if( std::holds_alternative<morgana::ptr>(to) ) return (size_t) std::stoll(val);
        else return val;
    }

    return r;
}
