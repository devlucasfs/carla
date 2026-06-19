// Isto armazena os dados do compilador
#pragma once

#include "common.hpp"
#include "libs/eva.hpp"
#include <cstring>
#include <stdexcept>
#include <string>

typedef struct CompilerParams {
public:
    std::string cwd;
    std::string command;
    std::string main;
    std::string target;
    bool verbose;

    bool ffi = false;
    std::string c_path;

    CompilerParams(std::string cwd, std::string command, std::string main, std::string target, bool verbose)
        : cwd(cwd),
          command(command),
          main(main),
          target(target),
          verbose(verbose) {};

    static struct CompilerParams format(int argc, char **argv) {
        char *cwd  = argv[0];
        char *command = argv[1];

        #define UNARY(flag, default, ...) \
        bool flag = default;

        #define BINARY(flag, default, ...) \
        char *flag = (char*) default;

        #define GET_MACRO(_1,_2,_3,_4,NAME,...) NAME
        #define X(...) GET_MACRO(__VA_ARGS__, BINARY, UNARY)(__VA_ARGS__)

        FLAGS_FIELDS

        #undef GET_MACRO
        #undef BINARY
        #undef UNARY
        #undef X

        if( std::string(command) == "build" || std::string(command) == "run" ) {
            eva reader("target.eva");
            try { auto m = reader.get<std::string>("target", "main");
                  main = (char*) m.c_str();
            } catch(std::runtime_error e) {}
        }

        for( int i = 2; i < argc; i++ ) {
            char *arg = argv[i];

            #define MAKE(flag)      \
            std::string str(#flag); \
            str = "-" + str;        \
            auto data = str.c_str()

            #define UNARY(flag,  _, desc)  {                       \
                MAKE(flag);                                        \
                if( std::strcmp(argv[i], data) == 0 ) flag = true; \
            }

            #define BINARY(flag, _, desc, complement) {                                   \
                MAKE(flag);                                                               \
                if( std::strcmp(argv[i], data) == 0 && (i + 1) < argc ) flag = argv[++i]; \
            }

            #define GET_MACRO(_1,_2,_3,_4,NAME,...) NAME
            #define X(...) GET_MACRO(__VA_ARGS__, BINARY, UNARY)(__VA_ARGS__)

            FLAGS_FIELDS

            #undef GET_MACRO
            #undef BINARY
            #undef UNARY
            #undef MAKE
            #undef X
        }

        return CompilerParams(cwd, command, main, target, verbose);
    }
} CompilerParams;
