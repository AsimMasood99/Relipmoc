#include "../include/relipmoc/token.hpp"
#include <iostream>
std::string tokenTypeToString(TokenType type) {
    switch (type) {
        // Keywords
        case TokenType::T_FUNCTION: return "T_FUNCTION";
        case TokenType::T_IF: return "T_IF";
        case TokenType::T_ELSE: return "T_ELSE";
        case TokenType::T_ELSE_IF: return "T_ELSE_IF";
        case TokenType::T_WHILE: return "T_WHILE";
        case TokenType::T_FOR: return "T_FOR";
        case TokenType::T_RETURN: return "T_RETURN";
        case TokenType::T_PRINT: return "T_PRINT";
        
        // Data types
        case TokenType::T_INT: return "T_INT";
        case TokenType::T_FLOAT: return "T_FLOAT";
        case TokenType::T_BOOL: return "T_BOOL";
        case TokenType::T_STRING: return "T_STRING";
        
        // Literals
        case TokenType::T_IDENTIFIER: return "T_IDENTIFIER";
        case TokenType::T_CONST_INT: return "T_CONST_INT";
        case TokenType::T_CONST_FLOAT: return "T_CONST_FLOAT";
        case TokenType::T_CONST_BOOL: return "T_CONST_BOOL";
        case TokenType::T_STRINGLIT: return "T_STRINGLIT";
        
        // Operators
        case TokenType::T_ASSIGNMENT_OPR: return "T_ASSIGNMENT_OPR";
        case TokenType::T_EQUALS_OPR: return "T_EQUALS_OPR";
        case TokenType::T_NOT_EQUALS_OPR: return "T_NOT_EQUALS_OPR";
        case TokenType::T_LESS_THAN_OPR: return "T_LESS_THAN_OPR";
        case TokenType::T_GREATER_THAN_OPR: return "T_GREATER_THAN_OPR";
        case TokenType::T_LESS_THAN_EQUAL_TO_OPR: return "T_LESS_THAN_EQUAL_TO_OPR";
        case TokenType::T_GREATER_THAN_EQUAL_TO_OPR: return "T_GREATER_THAN_EQUAL_TO_OPR";
        case TokenType::T_AND_OPR: return "T_AND_OPR";
        case TokenType::T_OR_OPR: return "T_OR_OPR";
        case TokenType::T_PLUS_OPR: return "T_PLUS_OPR";
        case TokenType::T_MINUS_OPR: return "T_MINUS_OPR";
        case TokenType::T_MULTIPLY_OPR: return "T_MULTIPLY_OPR";
        case TokenType::T_DIVIDE_OPR: return "T_DIVIDE_OPR";
        case TokenType::T_NOT: return "T_NOT";
        case TokenType::T_LEFT_SHIFT_OPR: return "T_LEFT_SHIFT_OPR";
        case TokenType::T_RIGHT_SHIFT_OPR: return "T_RIGHT_SHIFT_OPR";
        case TokenType::T_INCREMENT: return "T_INCREMENT";
        case TokenType::T_DECREMENT: return "T_DECREMENT";
        
        // Punctuation/Delimiters
        case TokenType::T_ROUND_BRACKET_OPEN: return "T_ROUND_BRACKET_OPEN";
        case TokenType::T_ROUND_BRACKET_CLOSE: return "T_ROUND_BRACKET_CLOSE";
        case TokenType::T_SQUARE_BRACKET_OPEN: return "T_SQUARE_BRACKET_OPEN";
        case TokenType::T_SQUARE_BRACKET_CLOSE: return "T_SQUARE_BRACKET_CLOSE";
        case TokenType::T_CURLY_BRACKET_OPEN: return "T_CURLY_BRACKET_OPEN";
        case TokenType::T_CURLY_BRACKET_CLOSE: return "T_CURLY_BRACKET_CLOSE";
        case TokenType::T_SEMICOLON: return "T_SEMICOLON";
        case TokenType::T_COMMA: return "T_COMMA";
        case TokenType::T_DOT: return "T_DOT";
        
        // Special tokens
        case TokenType::T_COMMENT: return "T_COMMENT";
        
        default: return "UNKNOWN_TOKEN";
    }
}

std::ostream& operator<<(std::ostream& os, const Token& token) {
    os << "<" << tokenTypeToString(token.type) << ", \"" << token.value << "\">";
    return os;
}