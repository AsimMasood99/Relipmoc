#include "../include/relipmoc/lexer.hpp"
#include <iostream>
Lexer::Lexer(const std::string &src) : source(src) {
    this->token_patterns = {
    {"Keyword",    std::regex(R"(\b(fn|if|else|elif|while|for|return|print|int|float|bool|string)\b)")},
    {"Identifier", std::regex(R"([a-zA-Z_][a-zA-Z0-9_]*)")},
    {"Float",      std::regex(R"([0-9]+\.[0-9]+)")},   
    {"Number",     std::regex(R"([0-9]+)")},           
    {"String",     std::regex(R"("(\\.|[^"])*")")},    
    {"Comment",    std::regex(R"(#.*)")},             
    {"Operator",   std::regex(R"(==|!=|<=|>=|\+\+|--|&&|\|\||[=+\-*/<>!&|()\[\]\{\}])")},
    {"Separators",  std::regex(R"(;|,|\.)")},
    {"Whitespace", std::regex(R"(\s+)")}
};

}

std::vector<Token> Lexer::tokenize() {
    std::vector<Token> tokens;
    std::string::const_iterator start = source.begin();
    std::string::const_iterator end = source.end();

    while (start != end) {
        bool matched = false;

        for (const auto &pattern : token_patterns) {
            std::smatch match;
            if (std::regex_search(start, end, match, pattern.second,
                                  std::regex_constants::match_continuous)) {
                // tokens.push_back({pattern.first, match.str()});
                std::cout << "Token: " << pattern.first
                          << ", Value: " << match.str() << std::endl;
                start = match[0].second; // advance iterator after the match
                matched = true;
                break;
            }
        }

        if (!matched) {
            std::cerr << "Unexpected token at: " << std::string(start, end)
                      << std::endl;
            break;
        }
    }

    return tokens;
}
