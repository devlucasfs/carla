#pragma once

#include "parser/ctx.hpp"
#include <string>
#include <vector>

namespace carla {
    struct Ns {
        std::string identifier;
        std::vector<pContext> body;

        Ns(std::string identifier, std::vector<pContext> body)
            : identifier(identifier),
              body(body) {};
    };
}
