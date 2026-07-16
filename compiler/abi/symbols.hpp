#pragma once

#include <sstream>
#include <string>
#include <vector>

namespace carla::abi {
    std::vector<std::string> namespaces;
    struct ns {
        static void entry(std::string identifier) {
            carla::abi::namespaces.push_back(identifier);
        }

        static void pop() {
            carla::abi::namespaces.pop_back();
        }
    };

    std::string function(std::string identifier) {
        if( identifier == "main" && namespaces.size() == 0 ) return identifier;
        std::stringstream ss; ss << "_C";
        for( auto ns : carla::abi::namespaces ) ss << 'N' << ns.size() << ns;
        ss << identifier.size() << identifier;
        return ss.str();
    }
}
