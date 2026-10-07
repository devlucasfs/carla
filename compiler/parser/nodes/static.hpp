#pragma once

#include <cstdint>
#include <string>
#include <variant>
namespace carla {
    using comptime_value = std::variant<std::monostate, std::string, std::int64_t, double, bool>;
    struct StaticValue {
        comptime_value data;
        StaticValue(comptime_value data) : data(data) {};
        StaticValue() = default;
        ~StaticValue() = default;
    };
}
