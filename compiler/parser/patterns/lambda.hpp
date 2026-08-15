#pragma once
#include "../nodes/lambda.hpp"
#include "../pattern.hpp"
#include "tokenizer/token_kind.hpp"
#include "type.hpp"
#include <cstdlib>
#include <tuple>

bool lambda(CARLA_PATTERN_ARGUMENTS) {
    CARLA_PATTERN_STARTS(bool, false);
    CARLA_GET_NEXT(first, _default);
    if( first.kind != Block ) CARLA_RETURN_DEFAULT;
    auto arguments = std::get<std::vector<pContext>>(first.content);
    std::vector<std::tuple<carla::Type, std::string>> t;

    size_t i = 0;
    while(i < arguments.size()) {
        auto [success, s, str] = typement(result, sym, &i, &arguments);
        if(! success ) CARLA_RETURN_DEFAULT;

        CARLA_INDEX_NEXT(identifier, _default, (&arguments), (&i));
        if( identifier.kind != Common ) CARLA_RETURN_DEFAULT;
        auto tk = std::get<Token>(identifier.content);
        if( tk.kind != IDENTIFIER ) CARLA_RETURN_DEFAULT;

        t.push_back({ carla::Type(str, s), tk.lexeme });

        if( (i + 1) < arguments.size() ) {
            CARLA_INDEX_NEXT(comma, _default, (&arguments), (&i));
            if( comma.kind != Common ) CARLA_RETURN_DEFAULT;

            auto tk = std::get<Token>(comma.content);
            if( tk.kind != COMMA ) CARLA_RETURN_DEFAULT;
        }
    }

    /* TODO: Attributes */

    CARLA_GET_NEXT(body, _default);
    if( body.kind != Block ) CARLA_RETURN_DEFAULT;
    auto vBody = std::get<std::vector<pContext>>(body.content);

    CARLA_PEEK_NEXT(semi, _default);
    if( semi.kind != Common ) CARLA_RETURN_DEFAULT;
    if( std::get<Token>(semi.content).kind != SEMICOLON ) CARLA_RETURN_DEFAULT;

    file_stack* fstack_cp = new file_stack(fstack);
    *result = carla::Lambda(t, vBody, static_cast<void*>(fstack_cp));
    return true;
}
