#include "../include/relipmoc/lexer.hpp"
#include "../include/relipmoc/token.hpp"
#include <fstream>
#include <iostream>
#include <vector>

int main() {
    std::ifstream file("../examples/sample.txt");
    if (!file) {
        std::cout << "Failed to open file.";
        return 1;
    }

    std::string code((std::istreambuf_iterator<char>(file)),
                        (std::istreambuf_iterator<char>()));

    std::vector<Token> tokens;
    // std::cout << code;
    Lexer lexer(code);
    tokens = lexer.tokenize();
    
    for (const auto& token : tokens) {
        std::cout << token << ",";
    }

    return 0;
}
