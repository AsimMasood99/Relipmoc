#include "../include/relipmoc/lexer.hpp"
#include "../include/relipmoc/token.hpp"
#include <fstream>
#include <iostream>
int main() {
    std::ifstream file("../examples/sample.txt");
    if (file) {
        std::string code((std::istreambuf_iterator<char>(file)),
                         (std::istreambuf_iterator<char>()));

        // std::cout << code;
        Lexer lexer(code);
        lexer.tokenize();
        // std::vector<Token> tokens = lexer.tokenize();
    } else {
        std::cout << "Failed to open file.";
    }

    return 0;
}
