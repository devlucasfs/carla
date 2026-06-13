#pragma once

#include "charset.hpp"
#include "common.hpp"
#include "params.hpp"
#include "compiler_outputs.hpp"
#include "precompiler.hpp"
#include "morgana/gen.hpp"
#include "tokenizer/scanner.hpp"
#include "tokenizer/token.hpp"
#include "parser/parser.hpp"
#include "parser/symbols.hpp"

#include <cstdlib>
#include <iomanip>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <regex>
#include <vector>

#ifdef _WIN32
# define NIL_FD " > NUL 2>&1 "
# define EXE_SUFFIX std::string(".exe")
#else
# define NIL_FD " > /dev/null 2>&1 "
# define EXE_SUFFIX std::string()
#endif

#define COMMANDS_FIELDS \
    X(version, "use to view the version of the compiler") \
    X(help, "use to list all available commands") \
    X(create, "use to create new projects") \
    X(init, "use to initialize a project") \
    X(build, "use to build the project as binary") \
    X(run, "use to run the project")

struct Commands {
    int status = 0;

    static bool help(CompilerParams& params) {
        std::cout << "\e[1;34mAvailable commands\e[0m:\n";
        constexpr int padding = 8;
        #define X(cmd, desc) \
        std::cout << "  \e[1;33m" << std::left << std::setw(padding) << #cmd << "\e[0m - \e[1;30m" << desc << "\e[0m\n";
        COMMANDS_FIELDS
        #undef X
        return 0;
    }

    static bool version(CompilerParams& params) {
        std::cout << "carla version " << CARLA_VERSION << " (" << CARLA_PS << ")\n";
        return 0;
    }

    static bool create(CompilerParams& params);
    static bool build(CompilerParams& params);
    static bool init(CompilerParams& params);
    static bool run(CompilerParams& params);

    Commands(CompilerParams& params) {
        #define X(cmd, desc) if( params.command == #cmd ) status = cmd(params);
        COMMANDS_FIELDS
        #undef X
    }
};

bool Commands::create(CompilerParams& params) {
    std::cout << Colorizer::DARK_GREY << Colorizer::BOLD << "Enter project name" << Colorizer::RESET << ":\n"
              << Colorizer::PURPLE << Colorizer::BOLD << " % " << Colorizer::RESET;
    std::string name;
    std::cin >> name;

    std::string regex_str = "^[a-z_]{1}[a-z0-9_]*";
    std::regex rgx(regex_str);
    if(! std::regex_match(name, rgx) ) {
        CompilerOutputs::Fatal("Invalid project name: " + name + ". Must follow the pattern: " + regex_str + "\n");
        return false;
    }

    CompilerOutputs::Log("Creating project...");

    std::string folder_path = std::filesystem::current_path().string() + "/" + name;
    std::filesystem::create_directory(folder_path);

    CompilerOutputs::ClearCurrentLine();
    CompilerOutputs::Log("Running init...");

    std::filesystem::current_path(folder_path);

    std::string init = "carla init " + std::string(NIL_FD);
    system(init.c_str());

    CompilerOutputs::ClearCurrentLine();
    CompilerOutputs::Log("Project created successfully!\n");

    return true;
}

bool Commands::run(CompilerParams& params) {
    std::filesystem::path absPathBin = std::filesystem::absolute("target/output" + EXE_SUFFIX);
    std::string runCommand = "./" + std::filesystem::relative(absPathBin.string()).string();
    std::cout << Colorizer::DARK_GREY << "   └─ " << Colorizer::BOLD_YELLOW << "Running " << Colorizer::RESET << std::endl;

    int result = std::system(runCommand.c_str());
    int exitCode = 0;

    #ifdef _WIN32
        exitCode = result;
    #else
        if( WIFEXITED(result) ) exitCode = WEXITSTATUS(result);
        else exitCode = result;
    #endif

    if( params.verbose ) {
        CompilerOutputs::ClearCurrentLine();
        CompilerOutputs::Log("Executable ran and left with " + std::to_string(exitCode) + "\n");
    }

    return true;
}

bool Commands::build(CompilerParams& params) {
    auto start = std::chrono::high_resolution_clock::now();

    /* checks if the file is accessible */
    std::ifstream file(params.main, std::ios::binary | std::ios::ate);
    if(! file.is_open() ) CompilerOutputs::Fatal("Your main file is not valid. Try use -m to define the newest file");

    std::streamsize size = file.tellg();
    file.seekg(0, std::ios::beg);

    std::vector<char> src(size);
    if(! file.read(src.data(), size) ) CompilerOutputs::Fatal("Your main file is not valid. Try use -m to define the newest file");

    Symt symbols;
    /* Lexical & Precompiler phase */
    std::vector<Token> tokens = Scanner::read(src, size);
    // std::vector<Token> tokens = precompiler(src, size, params);

    /* Parser phase */
    charset(symbols);
    auto irNodes = Parser::parse(symbols, tokens);

    /* Code Generation phase */
    std::string morgIR = generateMorganaCode(irNodes, symbols, false);

    /* Create target directory if it doesn't exist */
    auto targetDir = std::filesystem::current_path() / "target";
    if(! std::filesystem::exists(targetDir) ) std::filesystem::create_directory(targetDir);

    /* Write Morgana IR to target/output.morg */
    std::ofstream outFile("target/output.morg");
    if(! outFile.is_open() ) CompilerOutputs::Fatal("Failed to open output file target/output.morg");
    outFile << morgIR;
    outFile.close();
    if( outFile.fail() ) CompilerOutputs::Fatal("Failed to write Morgana IR to target/output.morg");

    std::filesystem::path absPath = std::filesystem::absolute("target/output.morg");

    /* calculate time of the **INTERNAL** compilation process */
    auto mid = std::chrono::high_resolution_clock::now();
    auto midMS = std::chrono::duration_cast<std::chrono::microseconds>(mid - start);

    float midSeconds = midMS.count() / 1000000.0;
    std::stringstream duration;
    duration << "Total " << Colorizer::BOLD_CYAN << "Carla" << Colorizer::RESET
            << " compilation proccess time: " << Colorizer::BOLD_GREEN
            << std::fixed << std::setprecision(2) << midSeconds << "s"
            << Colorizer::RESET << "\n";
    CompilerOutputs::Log(duration.str());

    std::cout << Colorizer::DARK_GREY << "└─ " << Colorizer::RESET << "Morgana Object generated "
            << Colorizer::BOLD << Colorizer::DARK_GREY << Colorizer::BOLD_YELLOW << " (not compiled yet)";

    /* Compile Morgana IR to object file using morgc silently */
    std::string flgs = ((params.target != "unknown") ? " -o " + params.target : "");
    if( params.verbose ) flgs += " -v";
    std::string morgcCommand = "morgana build -m " + absPath.string() + flgs;
    if( params.verbose ) CompilerOutputs::Warn("Runnig morgana as " + morgcCommand + "\n");

    FILE* pipe = popen(morgcCommand.c_str(), "r");
    if(! pipe ) {
        CompilerOutputs::Fatal("Fail when to open Morgana pipe.\n");
        return false;
    }

    char buffer[256];
    std::vector<std::string> lines;

    while( fgets(buffer, sizeof(buffer), pipe) != nullptr ) {
        std::string line(buffer);
        if(! line.empty() && line.back() == '\n') line.pop_back();
        lines.push_back(line);
    }

    int status_bruto = pclose(pipe);

    if( WEXITSTATUS(status_bruto) == 38 ) {
        std::string line = lines.at(0);

        for( int i = 0; i < 3; i++ ) {
            std::cout << "\033[1A\r";
            CompilerOutputs::ClearCurrentLine();
        }

        std::cout.flush();
        std::cout << line << std::endl;
        exit(0);
    }

    auto end = std::chrono::high_resolution_clock::now();
    auto ms = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    float seconds = ms.count() / 1000000.0;

    for( std::string line : lines )
    /* -> */ std::cout << line << std::endl;

    CompilerOutputs::ClearCurrentLine();
    std::cout << Colorizer::DARK_GREY << "   └─ " << Colorizer::RESET << "Total compilation proccess time: " << Colorizer::BOLD_YELLOW
              << std::fixed << std::setprecision(2) << seconds << "s"
              << Colorizer::RESET << "\n";

    return true;
}

bool Commands::init(CompilerParams& params) {
    std::string targetDir = std::filesystem::current_path() / "target.eva";
    std::ofstream target(targetDir, std::ios::out);
    if(! target.is_open() ) CompilerOutputs::Fatal("Failed to create target.eva");

    target <<
    "@target\n"
    "' Specify the file who your entrypoint is\n"
    "main: \"src/main.crl\"\n"
    "\n"
    "' You can use here to specify the output format.\n"
    "' Is available: \"native-bin\", \"static-archive\" or \"shared-object\"\n"
    "output: { format: \"native-bin\" }\n"
    "\n"
    "@extensors\n"
    "' You can put more git repositories here.\n"
    "' Each repository will be checked when you try to install an extensor.\n"
    "' \"git@github.com:Carla-Corp/extensors.git\" is the default (official) repository.\n"
    "repositories: [ \"git@github.com:Carla-Corp/extensors.git\" ]\n";

    target.close();

    auto srcDir = std::filesystem::current_path() / "src";
    std::filesystem::create_directory(srcDir);

    auto srcFile = srcDir / "main.crl";
    std::ofstream main(srcFile, std::ios::out);

    main <<
        "@_start\n"
        "void main = () {\n"
        "\tputs \"Hello, world\";\n"
        "}\n";

    main.close();

    CompilerOutputs::Log("Project initialized successfully!\n");
    return true;
}
