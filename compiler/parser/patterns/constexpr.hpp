#pragma once
#include "type.hpp"
#include "../pattern.hpp"
#include <iostream>
#include <optional>
#include <variant>

bool const_definition(CARLA_PATTERN_ARGUMENTS) {
    CARLA_PATTERN_STARTS(bool, false);
    (*index)++;

    auto [get_type_success, type, str] = typement(CARLA_PATTERN_EXPORT);
    if(! get_type_success ) *index = backup + 1;

    CARLA_GET_NEXT(id, _default);
    if( id.kind != Common ) CARLA_RETURN_DEFAULT;

    auto tk = std::get<Token>(id.content);
    if( tk.kind != IDENTIFIER ) CARLA_RETURN_DEFAULT;
    std::string identifier = tk.lexeme;

    CARLA_GET_NEXT(colon_equal, _default);
    if( colon_equal.kind != Common ) CARLA_RETURN_DEFAULT;

    tk = std::get<Token>(colon_equal.content);
    if( get_type_success && tk.kind != EQUAL ) CARLA_RETURN_DEFAULT;
    if( (! get_type_success) && tk.kind != COLON_EQUAL ) CARLA_RETURN_DEFAULT;

    pNode expr_;
    if(! expression(&expr_, sym, index, ctx) ) CARLA_RETURN_DEFAULT;

    auto expr = std::get<carla::Expr>(expr_);
    if(! expr.is_static ) CARLA_RETURN_DEFAULT;

    if( get_type_success ) {
        carla::Type decl_t(str, type);
        sym->addSymbol(identifier, carla::symbols::const_variable(decl_t, expr.data));

        result->~pNode();
        new(result) pNode(carla::Nop());

        (*index)--;
        return true;
    }

    std::optional<carla::Type> decl_t;

    if( std::holds_alternative<size_t>(expr.data) ) decl_t.emplace("int", morgana::integer(0));
    if( std::holds_alternative<std::string>(expr.data) ) decl_t.emplace("ascii", morgana::ptr());

    if(! decl_t.has_value() ) CARLA_RETURN_DEFAULT;

    sym->addSymbol(identifier, carla::symbols::const_variable(decl_t.value(), expr.data));

    result->~pNode();
    new(result) pNode(carla::Nop());

    (*index)--;
    return true;
}
