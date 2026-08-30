#pragma once

#include <iostream>
#include <stack>
#include <unordered_map>
#include <variant>
#include <vector>

#include "../parser/node.hpp"
#include "../parser/symbols.hpp"
#include "../parser/parser.hpp"

#include "../libs/morgana/builder.hpp"
#include "../libs/morgana.hpp"
#include "abi/symbols.hpp"
#include "compiler_outputs.hpp"
#include "parser/nodes/morgana/quote.hpp"
#include "parser/nodes/statement.hpp"
#include "parser/nodes/type.hpp"
#include "parser/pattern.hpp"
#include "tokenizer/token_kind.hpp"

enum reason_t { VAR_DECLARATION };
using variable_id = size_t;
using declaration_t = std::tuple<variable_id, morgana::type>;
std::stack<std::tuple<reason_t, std::variant<
    std::monostate,
    declaration_t
>>> stack_reason;

using var_map = std::unordered_map<std::string, std::tuple<size_t, carla::Type>>;

std::string generateMorganaCode(std::vector<pNode> nodes, Symt& symbols, bool internal) {
    Builder builder;
    Storage storage;
    storage.variable.push(0);
    var_map vmap;

    for( int index = 0; index < nodes.size(); index++ ) {
        pNode node = nodes[index];

        switch(node.index()) {
            case MORG_QUOTE: {
                auto quote = std::get<carla::morgana::Quote>(node);
                auto data =
                    (quote.kind == IF_TARGET)
                    ? "iftarget"
                    : "";

                builder << morgana::quote( quote.quote, data );

                if( nodes.size() <= (index + 1) || nodes[index + 1].index() != THEN )
                    CompilerOutputs::Fatal("@" + std::string(data) + " should be used before a THEN (`:`) operator.");

                if( nodes.size() <= (index + 2) )
                    CompilerOutputs::Fatal("after a THEN (`:`) operator, you should to use some expression.");

                std::vector<pNode> after { nodes[index + 2] };
                std::string final = generateMorganaCode(after, symbols, internal);
                builder << final + "\n}\n";
            } break;

            case COMPTIME_START: builder << morgana::comptime("_start"); break;
            case NS: {
                std::vector<pNode> statement;
                auto ns = std::get<carla::Ns>(node);
                Parser::checkSyntax(symbols, &statement, ns.body, false);

                carla::abi::ns::entry(ns.identifier);

                Context ctx;
                ctx << generateMorganaCode(statement, symbols, true);
                builder << ctx.string();

                carla::abi::ns::pop();
            } continue;

            case DECLARATION: {
                auto decl = std::get<carla::Decl>(node);

                if( decl.k == carla::Decl::Hopefull && (index + 1) < nodes.size() && nodes[index + 1].index() == LAMBDA ) {
                    auto lambda = std::get<carla::Lambda>(nodes[index + 1]);
                    std::vector<pNode> statement;

                    special_fstack = lambda.fstack_copy;
                    Parser::checkSyntax(symbols, &statement, lambda.body, false);

                    if( lambda.fstack_copy != NULL )
                    /* -> */ std::free(lambda.fstack_copy);

                    std::vector<morgana::type> types;
                    std::vector<std::string> identifiers;
                    for( auto& [type, identifier] : lambda.args ) {
                        types.push_back(type.morgana);
                        identifiers.push_back(identifier);
                    }

                    Context ctx;
                    ctx << generateMorganaCode(statement, symbols, true);
                    builder << morgana::function(&storage, carla::abi::function(decl.identifier), decl.type.morgana, types, ctx);
                    index++;
                    continue;
                }

                builder << morgana::alloc(&storage, decl.type.morgana);
                size_t alloc = (storage.variable.top() - 1);
                vmap.insert({ decl.identifier, { alloc, decl.type } });
                if( decl.k == carla::Decl::Hopefull ) stack_reason.push({
                    VAR_DECLARATION,
                    declaration_t { morgana::last(&storage, "alloc"), decl.type.morgana }
                });

                break;
            } break;
            case STATEMENT: {
                auto stmt = std::get<carla::Stmt>(node);
                switch(stmt.data) {
                    case carla::STMT_PUTS: {
                        // auto err = [](){ CompilerOutputs::Fatal("Expected a string static expression after puts statement"); };
                        // if( (index + 1) >= nodes.size() ) err();

                        // auto expr = std::get<carla::Expr>(nodes[index + 1]);
                        // if(! expr.is_static ) err();

                        // if(! std::holds_alternative<std::string>(expr.data) ) err();
                        // builder << morgana::puts(&storage, std::get<std::string>(expr.data));
                        // index++;
                    } break;
                }
            } break;
            case NOP: break;
        }
    }

    return builder.string();
};
