#pragma once

#include "../pattern.hpp"
#include "expression.hpp"
#include <iostream>
#include <stdexcept>
#include <variant>

#define NEED_BLOCK(t) *result = carla::t((*ctx)[++(*index)])

#define CARLA_MACROS_FIELDS \
    X(cast)

#include "../../macros/cast.hpp"

bool arg_macros(CARLA_PATTERN_ARGUMENTS, std::string macro);

bool macros(CARLA_PATTERN_ARGUMENTS, size_t macro) {
    CARLA_PATTERN_STARTS(bool, false);
    switch(macro) {
        case START: *result = carla::Start(); break;
        case CAST: return arg_macros(CARLA_PATTERN_EXPORT, "cast");
        default: CARLA_RETURN_DEFAULT;
    }
    (*index)++;
    return true;
}

bool arg_macros(CARLA_PATTERN_ARGUMENTS, std::string macro) {
    CARLA_PATTERN_STARTS(bool, false);
    (*index)++;

    CARLA_GET_NEXT(args, _default);
    if( args.kind != Block ) CARLA_RETURN_DEFAULT;

    auto block = std::get<std::vector<pContext>>(args.content);
    std::vector<carla::Expr> exprs{};

    for( size_t i = 0; i < block.size(); i++ ) {
        if( i != 0 ) {
            auto comma = block[i++];
            if( comma.kind != Common ) CARLA_RETURN_DEFAULT;

            auto tk = std::get<Token>(comma.content);
            if( tk.kind != COMMA ) CARLA_RETURN_DEFAULT;
        }

        pNode expr;
        if(! expression(&expr, sym, &i, &block, true) ) CARLA_RETURN_DEFAULT;

        if(! std::holds_alternative<carla::Expr>(expr) ) CARLA_RETURN_DEFAULT;

        exprs.push_back(std::get<carla::Expr>(expr));
        if( (i + 1) < block.size() ) i--;
    }

    carla::InterpreterResult v;
    try {
        #define X(id) else if( macro == #id ) v = assembler_##id(CARLA_PATTERN_EXPORT, exprs);
        if( false ) {}
        CARLA_MACROS_FIELDS
        else {}
        #undef X
    } catch(std::runtime_error err) {
        CompilerOutputs::Warn(err.what());
        std::cout << "\n";
        CARLA_RETURN_DEFAULT;
    }

    result->~pNode();
    new (result) pNode(v);

    return true;
}
