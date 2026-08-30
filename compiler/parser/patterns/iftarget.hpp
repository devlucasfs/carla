#pragma once

#include "../pattern.hpp"
#include "parser/ctx.hpp"
#include "parser/node.hpp"
#include "tokenizer/token_kind.hpp"

bool morgana_comptime(CARLA_PATTERN_ARGUMENTS, size_t macro) {
    CARLA_PATTERN_STARTS(bool, false);

    switch(macro) {
        case IF_TARGET: {
            (*index)++;
            CARLA_GET_NEXT(quote, _default);
            if( quote.kind != Common ) CARLA_RETURN_DEFAULT;

            auto tk = std::get<Token>(quote.content);
            if( tk.kind != STRING ) CARLA_RETURN_DEFAULT;

            result->~pNode();
            new (result) pNode(carla::morgana::Quote((TokenKind) macro, tk.lexeme));
            return true;
        } break;
        default: CARLA_RETURN_DEFAULT;
    }

    return false;
}
