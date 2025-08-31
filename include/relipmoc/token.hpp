#pragma once
#include <string>

enum class TokenType {
    // Keywords
    T_FUNCTION, // fn
    T_IF,       // if
    T_ELSE,     // else
    T_ELSE_IF,  // elif
    T_WHILE,    // while
    T_FOR,      // for
    T_RETURN,   // return
    T_PRINT,    // print

    // Data types
    T_INT,    // int
    T_FLOAT,  // float
    T_BOOL,   // bool
    T_STRING, // string

    // Literals
    T_IDENTIFIER,  // variable names
    T_CONST_INT,   // 123
    T_CONST_FLOAT, // 123.45
    T_CONST_BOOL,  // true/false (handled as keywords for now)
    T_STRINGLIT,   // "hello world"

    // Operators
    T_ASSIGNMENT_OPR,            // =
    T_EQUALS_OPR,                // ==
    T_NOT_EQUALS_OPR,            // !=
    T_LESS_THAN_OPR,             // <
    T_GREATER_THAN_OPR,          // >
    T_LESS_THAN_EQUAL_TO_OPR,    // <=
    T_GREATER_THAN_EQUAL_TO_OPR, // >=
    T_AND_OPR,                   // &&
    T_OR_OPR,                    // ||
    T_PLUS_OPR,                  // +
    T_MINUS_OPR,                 // -
    T_MULTIPLY_OPR,              // *
    T_DIVIDE_OPR,                // /
    T_NOT,                       // !
    T_LEFT_SHIFT_OPR,            // << (if you add this to regex)
    T_RIGHT_SHIFT_OPR,           // >> (if you add this to regex)

    // Increment/Decrement (from your regex)
    T_INCREMENT, // ++
    T_DECREMENT, // --

    // Punctuation/Delimiters
    T_ROUND_BRACKET_OPEN,   // (
    T_ROUND_BRACKET_CLOSE,  // )
    T_SQUARE_BRACKET_OPEN,  // [
    T_SQUARE_BRACKET_CLOSE, // ]
    T_CURLY_BRACKET_OPEN,   // {
    T_CURLY_BRACKET_CLOSE,  // }
    T_SEMICOLON,            // ;
    T_COMMA,                // ,
    T_DOT,                  // . (from your separators)

    // Special tokens
    T_COMMENT, // # comments
};

// Define Token class
class Token {
  public:
    TokenType type;
    std::string value;

    Token(TokenType type, const std::string value) : type(type), value(value) {}
    friend std::ostream &operator<<(std::ostream &os, const Token &token);
};

std::ostream& operator<<(std::ostream& os, const Token& token);