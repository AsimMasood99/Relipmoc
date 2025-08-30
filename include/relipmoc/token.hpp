#include <string>

enum class TokenType {
    T_FUNCTION,
    T_IF,
    T_ELSE,
    T_ELSE_IF,
    T_WHILE,
    T_FOR,
    T_RETURN,
    T_PRINT,
    T_INT,
    T_FLOAT,
    T_BOOL,
    T_STRING,
    T_DOUBLE_QUOTE,
    T_ROUND_BRACKET_OPEN,
    T_ROUND_BRACKET_CLOSE,
    T_SQUARE_BRACKET_OPEN,
    T_SQUARE_BRACKET_CLOSE,
    T_CURLY_BRACKET_OPEN,
    T_CURLY_BRACKET_CLOSE,
    T_COMMA,
    T_SEMICOLON,
    T_ASSIGNMENT_OPR,
    T_EQUALS_OPR,
    T_NOT_EQUALS_OPR,
    T_LESS_THAN_OPR,
    T_GREATER_THAN_OPR,
    T_LESS_THAN_EQUAL_TO_OPR,
    T_GREATER_THAN_EQUAL_TO_OPR,
    T_AND_OPR,
    T_OR_OPR,
    T_PLUS_OPR,
    T_MINUS_OPR,
    T_MULTIPLY_OPR,
    T_DIVIDE_OPR,
    T_NOT,
    T_LEFT_SHIFT_OPR,
    T_RIGHT_SHIFT_OPR,
    T_IDENTIFIER,
    T_CONST_INT,
    T_CONST_FLOAT,
    T_CONST_BOOL,
    T_STRINGLIT
};

// Define Token class
class Token {
public:
    TokenType type;
    std::string value;

    Token(TokenType type, const std::string value)
        : type(type), value(value) {}
};