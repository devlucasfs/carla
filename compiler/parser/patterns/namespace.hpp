#pragma once

#include "parser/ctx.hpp"
#include "parser/pattern.hpp"
#include "tokenizer/token.hpp"
#include "tokenizer/token_kind.hpp"

bool _namespace(CARLA_PATTERN_ARGUMENTS) {
    CARLA_PATTERN_STARTS(bool, false);
    (*index)++;

    CARLA_GET_NEXT(nIdentifier, _default);
    if( nIdentifier.kind != Common ) CARLA_RETURN_DEFAULT;

    auto tk = std::get<Token>(nIdentifier.content);
    if( tk.kind != IDENTIFIER ) CARLA_RETURN_DEFAULT;

    auto identifier = tk.lexeme;
    CARLA_GET_NEXT(nBody, _default);
    if( nBody.kind != Block ) CARLA_RETURN_DEFAULT;

    auto body = std::get<std::vector<pContext>>(nBody.content);

    result->~pNode();
    new (result) pNode(carla::Ns(identifier, body));
    return true;
}
