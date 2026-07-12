#pragma once
#include <string>

const std::string CARLA_VERSION = "26.0 alpha";
const std::string CARLA_PS = "still in development";

#define FLAGS_FIELDS                                                                               \
    X(target,  "uknown",   "use to define the output format based on your extensors", "extensor")  \
    X(main,    "main.crl", "use to define the entry file", "file")                                 \
    X(precomp, "norn",    "use to define the pre-compiler as will be used before the compilation", "precompiler") \
    X(verbose, false,     "use to define the compiler output as verbose")

#define COMMANDS_FIELDS                                   \
    X(version, "use to view the version of the compiler") \
    X(help,    "use to list all available commands")      \
    X(create,  "use to create new projects")              \
    X(init,    "use to initialize a project")             \
    X(build,   "use to build the project as binary")      \
    X(run,     "use to run the project")
