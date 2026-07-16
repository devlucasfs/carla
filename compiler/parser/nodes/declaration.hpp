#pragma once

#include <string>
#include "type.hpp"

namespace carla {
    struct Decl {
        enum kind { Hopeless, Hopefull, HopefullNontyped };
        std::string identifier;
        carla::Type type;
        kind k;

        Decl(std::string identifier, carla::Type type)
            : identifier(identifier),
              type(type) {}
    };
}
