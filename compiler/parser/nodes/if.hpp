#pragma once

#include <optional>
#include <vector>

#include "expression.hpp"

namespace carla {
    struct If {
        bool constant;
        carla::Expr expr;
        std::optional<carla::Type> result;
        std::vector<pContext> body;
    };
}
