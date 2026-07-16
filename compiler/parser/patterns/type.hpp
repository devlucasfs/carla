#pragma once

#include <optional>
#include <sstream>
#include <tuple>
#include <variant>
#include <vector>

#include "../symbols.hpp"
#include "../pattern.hpp"

#include "../../libs/morgana/types.hpp"


std::tuple<bool, morgana::type, std::string>
typement(CARLA_PATTERN_ARGUMENTS) {
    auto d = std::make_tuple(false, std::monostate(), std::string());
    CARLA_PATTERN_STARTS(auto, d);
    CARLA_GET_NEXT(first, _default);

    auto getptr = [&](std::tuple<bool, morgana::type, std::string> current) -> std::tuple<bool, morgana::type, std::string> {
        if( *index >= ctx->size() ) return current;

        int lvl, is = 0;
        CARLA_PEEK_NEXT(ptr, _default);
        if( ptr.kind == Common ) {
            auto tk = std::get<Token>(ptr.content);
            if( tk.kind != STAR ) return current;

            is = true;
            while(*index < ctx->size()) {
                CARLA_GET_NEXT(ptr, _default);
                if( ptr.kind != Common ) { (*index)--; break; };
                tk = std::get<Token>(ptr.content);
                if( tk.kind != STAR ) { (*index)--; break; };
                lvl++;
            }

            auto carla = std::get<2>(current);
            return { true, morgana::ptr(), carla + std::string(lvl, '*') };
        }

        return current;
    };

    std::stringstream carla;

    if( first.kind == Block ) {
        carla << '[';

        std::vector<morgana::type> tuple;
        auto body = std::get<std::vector<pContext>>(first.content);

        size_t i = 0;
        while(i < body.size()) {
            auto [success, s, str] = typement(result, sym, &i, &body);
            if(! success ) CARLA_RETURN_DEFAULT;
            tuple.push_back(s);
            carla << str;

            if( (i + 1) < body.size() ) {
                CARLA_INDEX_NEXT(comma, _default, (&body), (&i));
                if( comma.kind != Common ) CARLA_RETURN_DEFAULT;

                auto tk = std::get<Token>(comma.content);
                if( tk.kind != COMMA ) CARLA_RETURN_DEFAULT;
                carla << ", ";
            }
        }

        carla << ']';
        return getptr({ true, morgana::tuple(tuple), carla.str() });
    }

    auto tk = std::get<Token>(first.content);
    if( tk.kind != IDENTIFIER ) CARLA_RETURN_DEFAULT;

    auto type = sym->findSymbol(tk.lexeme);
    if( type == nullptr ) CARLA_RETURN_DEFAULT;
    if(! std::holds_alternative<morgana::type>(*type) ) CARLA_RETURN_DEFAULT;

    return getptr({ true, std::get<morgana::type>(*type), tk.lexeme });
}
