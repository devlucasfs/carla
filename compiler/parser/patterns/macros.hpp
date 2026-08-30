#pragma once

#include "../pattern.hpp"

#define NEED_BLOCK(t) *result = carla::t((*ctx)[++(*index)])

#define CARLA_MACROS_FIELDS \
    X(cast)

bool arg_macros(CARLA_PATTERN_ARGUMENTS, std::string macro);

bool macros(CARLA_PATTERN_ARGUMENTS, size_t macro) {
    CARLA_PATTERN_STARTS(bool, false);
    switch(macro) {
        case START: *result = carla::Start(); break;
        // case CAST: return arg_macros(CARLA_PATTERN_EXPORT, "cast");
        default: CARLA_RETURN_DEFAULT;
    }
    (*index)++;
    return true;
}
