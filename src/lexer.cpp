#include "../include/relipmoc/lexer.hpp"
#include <iostream>
Lexer::Lexer(const std::string &src) : source(src) {
    this->token_patterns = {
        {"Keyword",
         std::regex(
             R"(\b(fn|if|else|elif|while|for|return|print|int|float|bool|string)\b)")},
        {"Identifier", std::regex(R"([a-zA-Z_][a-zA-Z0-9_]*)")},
        {"Float", std::regex(R"([0-9]+\.[0-9]+)")},
        {"Number", std::regex(R"([0-9]+)")},
        {"String", std::regex(R"("(\\.|[^"])*")")},
        {"Comment", std::regex(R"(#.*)")},
        {"Operator",
         std::regex(R"(==|!=|<=|>=|\+\+|--|&&|\|\||[=+\-*/<>!&|()\[\]\{\}])")},
        {"Separators", std::regex(R"(;|,|\.)")},
        {"Whitespace", std::regex(R"(\s+)")}};

    init_keyword_map();
    init_operator_map();
}

void Lexer::init_keyword_map() {
    keyword_map = {
        {"fn", TokenType::T_FUNCTION},   {"if", TokenType::T_IF},
        {"else", TokenType::T_ELSE},     {"elif", TokenType::T_ELSE_IF},
        {"while", TokenType::T_WHILE},   {"for", TokenType::T_FOR},
        {"return", TokenType::T_RETURN}, {"print", TokenType::T_PRINT},
        {"int", TokenType::T_INT},       {"float", TokenType::T_FLOAT},
        {"bool", TokenType::T_BOOL},     {"string", TokenType::T_STRING}};
}

void Lexer::init_operator_map() {
    operator_map = {{"=", TokenType::T_ASSIGNMENT_OPR},
                    {"==", TokenType::T_EQUALS_OPR},
                    {"!=", TokenType::T_NOT_EQUALS_OPR},
                    {"<", TokenType::T_LESS_THAN_OPR},
                    {">", TokenType::T_GREATER_THAN_OPR},
                    {"<=", TokenType::T_LESS_THAN_EQUAL_TO_OPR},
                    {">=", TokenType::T_GREATER_THAN_EQUAL_TO_OPR},
                    {"&&", TokenType::T_AND_OPR},
                    {"||", TokenType::T_OR_OPR},
                    {"+", TokenType::T_PLUS_OPR},
                    {"-", TokenType::T_MINUS_OPR},
                    {"*", TokenType::T_MULTIPLY_OPR},
                    {"/", TokenType::T_DIVIDE_OPR},
                    {"!", TokenType::T_NOT},
                    {"++", TokenType::T_INCREMENT},
                    {"--", TokenType::T_DECREMENT},
                    {"(", TokenType::T_ROUND_BRACKET_OPEN},
                    {")", TokenType::T_ROUND_BRACKET_CLOSE},
                    {"[", TokenType::T_SQUARE_BRACKET_OPEN},
                    {"]", TokenType::T_SQUARE_BRACKET_CLOSE},
                    {"{", TokenType::T_CURLY_BRACKET_OPEN},
                    {"}", TokenType::T_CURLY_BRACKET_CLOSE},
                    {";", TokenType::T_SEMICOLON},
                    {",", TokenType::T_COMMA},
                    {".", TokenType::T_DOT}};
}

std::vector<Token> Lexer::tokenize() {
    std::vector<Token> tokens;
    std::string::const_iterator start = source.begin();
    std::string::const_iterator end = source.end();

    while (start != end) {
        bool matched = false;

        for (const auto &pattern : token_patterns) {
            std::smatch match;
            
            if (std::regex_search(start, end, match, pattern.second, std::regex_constants::match_continuous)) {
                
                if(pattern.first == "Whitespace" || pattern.first == "Comment") {
                    // Not tokenizing whitespace. 
                    start = match[0].second; // advance iterator after the match
                    matched = true;
                    break;
                }
                else if(pattern.first == "Keyword") {
                    std::string keyword = match.str();
                    if (keyword_map.find(keyword) != keyword_map.end()) {
                        tokens.push_back(Token{keyword_map[keyword], keyword}); // Fixed: was operator_map
                    }
                    start = match[0].second; // Added: advance iterator
                    matched = true;
                    break;
                }
                else if(pattern.first == "Operator") {
                    std::string op = match.str();
                    if (operator_map.find(op) != operator_map.end()) {
                        tokens.push_back(Token{operator_map[op], op});
                    }
                    start = match[0].second; // Added: advance iterator
                    matched = true;
                    break;
                }
                else if(pattern.first == "Identifier") {
                    tokens.push_back(Token{TokenType::T_IDENTIFIER, match.str()});
                    start = match[0].second; // Added: advance iterator
                    matched = true;
                    break;
                }
                else if(pattern.first == "Number") {
                    tokens.push_back(Token{TokenType::T_CONST_INT, match.str()});
                    start = match[0].second; // Added: advance iterator
                    matched = true;
                    break;
                }
                else if(pattern.first == "Float") {
                    tokens.push_back(Token{TokenType::T_CONST_FLOAT, match.str()});
                    start = match[0].second; // Added: advance iterator
                    matched = true;
                    break;
                }
                else if(pattern.first == "String") {
                    tokens.push_back(Token{TokenType::T_STRINGLIT, match.str()});
                    start = match[0].second; // Added: advance iterator
                    matched = true;
                    break;
                }
                else if(pattern.first == "Separators") {
                    std::string sep = match.str();
                    if (operator_map.find(sep) != operator_map.end()) {
                        tokens.push_back(Token{operator_map[sep], sep});
                    }
                    start = match[0].second; // Added: advance iterator
                    matched = true;
                    break;
                }
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