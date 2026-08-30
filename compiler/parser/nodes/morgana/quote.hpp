#pragma once

#include "tokenizer/token_kind.hpp"

namespace carla::morgana {
    struct Quote {
        TokenKind kind;
        std::string quote;

        Quote(TokenKind kind, std::string quote)
            : kind(kind),
              quote(quote) {};
    };
}
