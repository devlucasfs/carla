#pragma once
#include <exception>
#include <stdexcept>
#include <variant>

struct numeric {
public:
    using integer = long long;
    using decimal = double;

private:
    std::variant<integer, decimal> data;

    enum Kind : int {
        lhs_decimal = 1 << 0, lhs_integer = 1 << 1,
        rhs_decimal = 1 << 2, rhs_integer = 1 << 3
    };

public:
    template<typename T>
    T value() {
        if constexpr (! (std::is_same_v<T, decimal> | std::is_same_v<T, integer>) )
            throw std::runtime_error("Fail then read the `numeric` value.");

        if( std::holds_alternative<decimal>(this->data) ) return std::get<decimal>(this->data);
        return std::get<integer>(this->data);
    }

    template<typename T>
    bool is() {
        if constexpr (! (std::is_same_v<T, decimal> | std::is_same_v<T, integer>) )
            throw std::runtime_error("Fail then read the `numeric` value.");

        return std::holds_alternative<T>(this->data);
    }

    numeric() = default;

    #define TABLE_DEFAULT_ALLOWED_TYPES                                                           \
        X(double) X(long long)

    #define X(type)                                                                               \
    numeric(const type value) : data(value) {}                                                    \
    numeric& operator=(const type value) {                                                        \
        this->data = value;                                                                       \
        return *this;                                                                             \
    }                                                                                             \

    TABLE_DEFAULT_ALLOWED_TYPES

    #undef X

    #define TABLE_ARITHMETIC_OPERATIONS                                                           \
    X(-) X(+) X(*) X(/)

    #define TABLE_BITWISE_OPERATIONS                                                              \
    X_BITWISE(&) X_BITWISE(|) X_BITWISE(^) X_BITWISE(<<) X_BITWISE(>>)

    #define Y(flags, left_type, right_type, op)                                                   \
    case flags: {                                                                                 \
        auto lhs_val = std::get<left_type>(this->data);                                           \
        auto rhs_val = std::get<right_type>(rhs_ref.data);                                        \
        result.data = lhs_val op rhs_val;                                                         \
        return result;                                                                            \
    }

    #define Y_UNARY(flags, type, op)                                                              \
    case flags: {                                                                                 \
        auto val = std::get<type>(this->data);                                                    \
        result.data = op val;                                                                     \
        return result;                                                                            \
    }

    #define X(op)                                                                                 \
    numeric operator op(const numeric& rhs_ref) const {                                           \
        int lhs_kind = std::holds_alternative<integer>(data) ? lhs_integer : lhs_decimal;         \
        int rhs_kind = std::holds_alternative<integer>(rhs_ref.data) ? rhs_integer : rhs_decimal; \
        int data_kind = lhs_kind | rhs_kind;                                                      \
        numeric result;                                                                           \
        switch(data_kind) {                                                                       \
            Y(lhs_integer | rhs_integer, integer, integer, op)                                    \
            Y(lhs_decimal | rhs_decimal, decimal, decimal, op)                                    \
            Y(lhs_integer | rhs_decimal, integer, decimal, op)                                    \
            Y(lhs_decimal | rhs_integer, decimal, integer, op)                                    \
            default: throw std::runtime_error("Can't operate in `numeric` value.");               \
        }                                                                                         \
    }

    numeric operator%(const numeric& rhs_ref) const {
        if( std::holds_alternative<decimal>(this->data) ||
            std::holds_alternative<decimal>(rhs_ref.data)
        ) throw std::runtime_error("Modulo operation is only supported for integers");
        numeric result;
        result.data = std::get<integer>(this->data) % std::get<integer>(rhs_ref.data);
        return result;
    }

    #define X_BITWISE(op)                                                                         \
    numeric operator op(const numeric& rhs_ref) const {                                           \
        if( std::holds_alternative<decimal>(this->data) ||                                        \
            std::holds_alternative<decimal>(rhs_ref.data)                                         \
        ) throw std::runtime_error("Bitwise operations are only supported for integers");         \
        int lhs_kind = lhs_integer;                                                               \
        int rhs_kind = rhs_integer;                                                               \
        int data_kind = lhs_kind | rhs_kind;                                                      \
        numeric result;                                                                           \
        switch(data_kind) {                                                                       \
            Y(lhs_integer | rhs_integer, integer, integer, op)                                    \
            default: throw std::runtime_error("Can't perform bitwise operation.");                \
        }                                                                                         \
    }

    TABLE_ARITHMETIC_OPERATIONS;
    TABLE_BITWISE_OPERATIONS;

    numeric operator~() const {
        if( std::holds_alternative<decimal>(this->data) ) {
            throw std::runtime_error("Bitwise NOT is only supported for integers");
        }

        numeric result;
        int kind = std::holds_alternative<integer>(data) ? lhs_integer : lhs_decimal;
        switch(kind) {
            Y_UNARY(lhs_integer, integer, ~)
            default: throw std::runtime_error("Can't perform bitwise NOT.");
        }
        return result;
    }

    #define X_COMPOUND(op)                                                                        \
    numeric& operator op##=(const numeric& rhs_ref) {                                             \
        *this = *this op rhs_ref;                                                                 \
        return *this;                                                                             \
    }

    X_COMPOUND(+) X_COMPOUND(-) X_COMPOUND(*) X_COMPOUND(/) X_COMPOUND(%)
    X_COMPOUND(&) X_COMPOUND(|) X_COMPOUND(^) X_COMPOUND(<<) X_COMPOUND(>>)

    #undef X
    #undef Y
    #undef Y_UNARY
    #undef X_BITWISE
    #undef X_COMPOUND
};
