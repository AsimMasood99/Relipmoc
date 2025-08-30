#include <iostream>
#include <unordered_map>
#include <string>


//Other than below tokens T_indentifier("string")
//T_CONST_NUM_VALUE(Value)

std::unordered_map<std::string, std::string> tokenizationMap = {
    // Conditional
    {"if", "T_IF"},
    {"else", "T_ELSE"},
    //Loop
    {"while", "T_WHILE"},
    {"for", "T_FOR"},
    {"in", "T_IN"},
    {"break", "T_BREAK"},
    {"continue", "T_CONTINUE"},
    //Data Types
    {"int", "T_INT"},
    {"float", "T_FLOAT"},
    {"double", "T_DOUBLE"},
    {"char", "T_CHAR"},
    {"string", "T_STRING"},
    {"void", "T_VOID"},
    {"bool", "T_BOOL"},
    {"null", "T_NULL"},
    //Function
    {"fn", "T_FUNC"},
    {"return", "T_RETURN"},
    //Operators
    {"<", "T_LESS"},
    {">", "T_GREATER"},
    {"<=", "T_LESS_EQUAL"},
    {">=", "T_GREATER_EQUAL"},
    {"==", "T_IS_EQUAL"},
    {"!=", "T_IS_NOT_EQUAL"},
    {"+", "T_PLUS"},
    {"-", "T_MINUS"},
    {"*", "T_MULTIPLY"},
    {"/", "T_DIVIDE"},
    {"%", "T_MODULUS"},
    {"&&", "T_AND"},
    {"||", "T_OR"},
    {"!", "T_NOT"},
    {"=", "T_ASSIGN"},
    // Bitwise Operators
    {"&", "T_BITWISE_AND"},
    {"|", "T_BITWISE_OR"},
    {"^", "T_BITWISE_XOR"},
    {"~", "T_BITWISE_NOT"},
    {"<<", "T_SHIFT_LEFT"},
    {">>", "T_SHIFT_RIGHT"},
    // Brackets
    {"(", "T_LEFT_PAREN"},
    {")", "T_RIGHT_PAREN"},
    {"{", "T_LEFT_BRACE"},
    {"}", "T_RIGHT_BRACE"},
    {"[", "T_LEFT_BRACKET"},
    {"]", "T_RIGHT_BRACKET"},
    //Misc
    {";", "T_SEMICOLON"},
    {",", "T_COMMA"},
};