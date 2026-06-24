#pragma once

#include "../pattern.hpp"
#include "compiler_outputs.hpp"
#include "parser/ctx.hpp"
#include "parser/node.hpp"
#include "parser/nodes/expression.hpp"
#include "parser/patterns/expression.hpp"
#include "tokenizer/token.hpp"
#include "tokenizer/token_kind.hpp"
#include "utils/numeric.hpp"
#include <cstddef>
#include <iostream>
#include <variant>
#include <vector>

enum _1e { ANDOP, OROP, NOTOP, NOTHINGOP };

bool nullable(pNode& data) {
    if( std::holds_alternative<carla::Expr>(data) ) return false;

}

bool _if(CARLA_PATTERN_ARGUMENTS) {
    CARLA_PATTERN_STARTS(bool, false);

    (*index)++;
    CARLA_PEEK_NEXT(first, _default);
    if( first.kind == Block ) CARLA_RETURN_DEFAULT;

    auto token = std::get<Token>(first.content);

    if( token.kind == _CONSTEXPR ) {
        (*index)++;
        bool obvious = false;
        CARLA_GET_NEXT(expr, _default);
        auto blocks = std::get<std::vector<pContext>>(expr.content);

        _1e flag = NOTHINGOP;
        size_t position = 0;
        while(position < blocks.size()) {
            pNode lhs;
            std::cout << "\n\nchegou aqui\n\n";
            if(! expression(&lhs, sym, &position, &blocks) ) CARLA_RETURN_DEFAULT;
            if( std::holds_alternative<carla::Expr>(lhs) ) CARLA_RETURN_DEFAULT;

            if( position >= blocks.size() )
            switch(flag) {
                case ANDOP:     obvious = obvious && nullable(lhs); break;
                case OROP:      obvious = obvious || nullable(lhs); break;
                case NOTHINGOP: obvious =            nullable(lhs); break;
                default: break;
            };

            CARLA_GET_NEXT(comparator, _default);
            if( comparator.kind == Common ) CARLA_RETURN_DEFAULT;

            auto mhs = std::get<Token>(comparator.content);

            pNode rhs;
            if(! expression(&rhs, sym, &position, &blocks) ) CARLA_RETURN_DEFAULT;
            if( std::holds_alternative<carla::Expr>(rhs) ) CARLA_RETURN_DEFAULT;

            auto ldata = std::get<carla::Expr>(lhs),
                 rdata = std::get<carla::Expr>(rhs);


            if(! (ldata.is_static && rdata.is_static) ) CARLA_RETURN_DEFAULT;

            std::string lstr, rstr;
            if( std::holds_alternative<numeric>(ldata.data) ) lstr = "numeric";
            if( std::holds_alternative<numeric>(rdata.data) ) rstr = "numeric";

            if( std::holds_alternative<std::string>(ldata.data) ) lstr = "asciz";
            if( std::holds_alternative<std::string>(rdata.data) ) rstr = "asciz";

            switch(mhs.kind) {
                case EQUAL_EQUAL: {
                    if( lstr != rstr ) {
                        CompilerOutputs::Warn("Use-less constexpr if. A(an) " + lstr + " will never be equals to a(an) " + rstr);
                        break;
                    }

                    bool equals;
                    if( lstr == "asciz" ) {
                        auto lsubdata = std::get<std::string>(ldata.data),
                             rsubdata = std::get<std::string>(rdata.data);

                        equals = lsubdata == rsubdata;
                    }

                    if( lstr == "asciz" ) {
                        auto lsubdata = std::get<numeric>(ldata.data),
                             rsubdata = std::get<numeric>(rdata.data);

                        equals = lsubdata.value<double>() == lsubdata.value<double>();
                    }

                    switch(flag) {
                        case ANDOP:     obvious = obvious && equals; break;
                        case OROP:      obvious = obvious || equals; break;
                        case NOTHINGOP: obvious =            equals; break;
                        default: break;
                    };

                    std::cout << "\n\nevaluation: " << obvious << "\n\n";
                } break;
            }


            if( position >= blocks.size() ) break;
        }
    }

    return true;
}
