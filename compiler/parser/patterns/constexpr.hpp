#pragma once
#include "parser/ctx.hpp"
#include "parser/nodes/expression.hpp"
#include "parser/nodes/type.hpp"
#include "parser/patterns/expression.hpp"
#include "tokenizer/token_kind.hpp"
#include "type.hpp"
#include "../pattern.hpp"
#include "expression.hpp"
#include "../../charset.hpp"
#include "utils/numeric.hpp"
#include <iostream>
#include <optional>
#include <variant>
#include <vector>

bool const_definition(CARLA_PATTERN_ARGUMENTS) {
    CARLA_PATTERN_STARTS(bool, false);
    (*index)++;
    auto [get_type_success, type, str] = typement(CARLA_PATTERN_EXPORT);
    if(! get_type_success ) *index = backup + 1;

    CARLA_GET_NEXT(id, _default);
    if( id.kind != Common ) CARLA_RETURN_DEFAULT;

    auto tk = std::get<Token>(id.content);
    if( tk.kind != IDENTIFIER ) CARLA_RETURN_DEFAULT;
    std::string identifier = tk.lexeme;

    CARLA_GET_NEXT(colon_equal, _default);
    if( colon_equal.kind != Common ) CARLA_RETURN_DEFAULT;

    tk = std::get<Token>(colon_equal.content);
    if( get_type_success && tk.kind != EQUAL ) CARLA_RETURN_DEFAULT;

    if(! get_type_success ) {
        if( tk.kind != COLON_EQUAL ) {
            CARLA_RETURN_DEFAULT;
        };

        CARLA_PEEK_NEXT(next, _default);
        if( next.kind != Common ) CARLA_RETURN_DEFAULT;

        auto token = std::get<Token>(next.content);
        switch(token.kind) {
            case LAYOUT: {
                (*index)++;

                pNode tData;
                auto [success, morg, carl] = typement(&tData, sym, index, ctx);
                if(! success ) CARLA_RETURN_DEFAULT;

                std::vector<int> detach;

                CARLA_GET_NEXT(after, _default);
                if( after.kind != Common ) CARLA_RETURN_DEFAULT;

                auto token = std::get<Token>(after.content);
                switch(token.kind) {
                    case DETACH: {
                        CARLA_GET_NEXT(block, _default);
                        if( block.kind != Block ) CARLA_RETURN_DEFAULT;

                        auto data = std::get<std::vector<pContext>>(block.content);
                        size_t i = 0;
                        for( size_t i = 0; i < data.size(); i++ ) {
                            if( i != 0 ) {
                                auto comma = data[i++];
                                if( comma.kind != Common ) CARLA_RETURN_DEFAULT;

                                auto tk = std::get<Token>(comma.content);
                                if( tk.kind != COMMA ) CARLA_RETURN_DEFAULT;
                            }

                            pNode nExpr;
                            if(! expression(&nExpr, sym, &i, &data, true) ) CARLA_RETURN_DEFAULT;

                            if(! std::holds_alternative<carla::Expr>(nExpr) ) CARLA_RETURN_DEFAULT;

                            auto expr = std::get<carla::Expr>(nExpr);
                            if( std::holds_alternative<numeric>(expr.data) ) {
                                auto num = std::get<numeric>(expr.data);
                                detach.push_back(num.value<numeric::integer>());
                            }
                            if( (i + 1) < data.size() ) i--;
                        }

                        CARLA_GET_NEXT(semi, _default);
                        if( semi.kind != Common ) CARLA_RETURN_DEFAULT;

                        auto token = std::get<Token>(semi.content);
                        if( token.kind != SEMICOLON ) CARLA_RETURN_DEFAULT;

                        std::cout << "\n\nchegou aqui\n\n";

                        std::cout << "\n\ntamanho de detach: " << detach.size() << "\n";

                        carla::Type t(carl, morg);
                        sym->addSymbol(identifier, carla::symbols::const_layout(t, detach));
                        return true;
                    } break;

                    case SEMICOLON: {
                        carla::Type t(carl, morg);
                        sym->addSymbol(identifier, carla::symbols::const_layout(t, detach));
                        return true;
                    } break;

                    default: CARLA_RETURN_DEFAULT;
                }
            } break;
            default: break;
        }
    }

    pNode expr_;
    if(! expression(&expr_, sym, index, ctx) ) CARLA_RETURN_DEFAULT;

    auto expr = std::get<carla::Expr>(expr_);
    if(! expr.is_static ) CARLA_RETURN_DEFAULT;

    if( get_type_success ) {
        carla::Type decl_t(str, type);
        sym->addSymbol(identifier, carla::symbols::const_variable(decl_t, expr.data));

        result->~pNode();
        new(result) pNode(carla::Nop());

        (*index)--;
        return true;
    }

    std::optional<carla::Type> decl_t;

    if( std::holds_alternative<numeric>(expr.data) ) decl_t.emplace("int", morgana::integer(0));
    if( std::holds_alternative<std::string>(expr.data) ) decl_t.emplace(carla::charset::utf8, morgana::ptr());

    if(! decl_t.has_value() ) CARLA_RETURN_DEFAULT;

    sym->addSymbol(identifier, carla::symbols::const_variable(decl_t.value(), expr.data));

    result->~pNode();
    new(result) pNode(carla::Nop());

    (*index)--;
    return true;
}
