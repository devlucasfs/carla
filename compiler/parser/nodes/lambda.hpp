#pragma once

#include "type.hpp"
#include "../ctx.hpp"
#include <string>

namespace carla {
    struct Lambda {
        std::vector<std::tuple<carla::Type, std::string>> args;
        std::vector<pContext> body;
        void* fstack_copy;

        Lambda(std::vector<std::tuple<carla::Type, std::string>> args, std::vector<pContext> body)
            : args(std::move(args)),
              body(std::move(body)),
              fstack_copy(NULL) {}

        Lambda(std::vector<std::tuple<carla::Type, std::string>> args, std::vector<pContext> body, void *fstack)
            : args(std::move(args)),
              body(std::move(body)),
              fstack_copy(fstack) {}

        ~Lambda() = default;
    };
};
