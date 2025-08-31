#ifndef LEXER_HPP
#define LEXER_HPP

#include <iostream>
#include <fstream>
#include <vector>
#include <optional>
#include <algorithm>

using namespace std;

// Function declarations
// string get_code();
// bool find_delim(char c);
// vector<Token> lexer(const string& code);

#endif // LEXER_HPP

#include "lexer.hpp"
#include <sstream>

bool try_parse_double(const string& str) {
    try {
        stod(str);
        return true;
    } catch (const invalid_argument& ia) {
        return false;
    } catch (const out_of_range& oor) {
        return false;
    }
}


string get_code() {
    // TODO: allow custom file path
    ifstream file("data/code.txt");
    if (!file.is_open()) {
        cerr << "Error opening file!" << endl;
        return ""; // Or handle the error in a more appropriate way
    }

    stringstream buffer;
    buffer << file.rdbuf();
    return buffer.str();
}

bool find_delim(char c) {
    return isspace(c) ||
           (c == '(' || c == ')' || c == '[' || c == ']' || c == '{' || c == '}' ||
            c == ',' || c == ';' || c == '"' || c == '=' || c == '!' || c == '<' ||
            c == '>' || c == '&' || c == '|' || c == '+' || c == '-' || c == '*' ||
            c == '/' || c == '#');
}

vector<Token> lex(const string& code) {
    vector<Token> tokens;
    string trimmed_code = code;
    
    cout<<"before trimming"<<trimmed_code.length()<<"\n";
    
    // Trim leading and trailing whitespace
    size_t first = trimmed_code.find_first_not_of(" \t\n\r");
    if (string::npos == first) {
        return tokens; // Empty string
    }
    size_t last = trimmed_code.find_last_not_of(" \t\n\r");
    trimmed_code = trimmed_code.substr(first, (last - first + 1));

    cout<<"After trimming: "<<trimmed_code.length()<<"\n";
    cout<<"trimmed string: "<<trimmed_code<<"\n";
    
    size_t curr = 0;
    bool string_lit = false;

    while (curr < trimmed_code.length()) {
        // Skip whitespace
        while (curr < trimmed_code.length() && isspace(trimmed_code[curr])) {
            curr++;
        }

        size_t idx;
        if (string_lit) {
            size_t _idx = curr;
            while (_idx < trimmed_code.length()) {
                size_t quotePos = trimmed_code.find('"', _idx);

                if (quotePos == std::string::npos) {
                    throw std::runtime_error("Unclosed string literal");
                }

                // Check for escaped double quotes
                size_t backslashCount = 0;
                for (size_t checkPos = quotePos - 1; checkPos >= _idx; --checkPos) {
                    if (trimmed_code[checkPos] == '\\') {
                        backslashCount++;
                    } else {
                        break;
                    }
                }

                if (backslashCount % 2 == 0) {
                    idx = quotePos;
                    break;
                } else {
                    _idx = quotePos + 1; // Move past the escaped quote and continue searching
                }
            }
        } else {
            size_t found = string::npos;
            for (size_t i = curr; i < trimmed_code.length(); ++i) {
                if (find_delim(trimmed_code[i])) {
                    found = i;
                    break; // Stop at the first delimiter found
                }
            }

            if (found == string::npos) {
                idx = trimmed_code.length();
            } else {
                idx = found;
            }
            if (idx == curr) {
                idx++; // Delimiters are 1 length
            }
        }
        int length_to_take = idx-curr;
        string substr = trimmed_code.substr(curr, length_to_take);

        if (substr == "#") {
            size_t newline_pos = trimmed_code.find('\n', curr);
            if (newline_pos != string::npos) {
                curr = newline_pos + 1;
                continue;
            } else {
                break;
            }
        }

        cout<<"substr length<< "<<substr.length()<<" \n";
        
        if (substr == "fn") {
            tokens.push_back({TokenType::T_FUNCTION, ""});
        } else if (substr == "if") {
            tokens.push_back({TokenType::T_IF, ""});
        } else if (substr == "else") {
            tokens.push_back({TokenType::T_ELSE, ""});
        } else if (substr == "elif") {
            tokens.push_back({TokenType::T_ELSE_IF, ""});
        } else if (substr == "while") {
            tokens.push_back({TokenType::T_WHILE, ""});
        } else if (substr == "for") {
            tokens.push_back({TokenType::T_FOR, ""});
        } else if (substr == "return") {
            tokens.push_back({TokenType::T_RETURN, ""});
        } else if (substr == "print") {
            tokens.push_back({TokenType::T_PRINT, ""});
        } else if (substr == "int") {
            tokens.push_back({TokenType::T_INT, ""});
        } else if (substr == "float") {
            tokens.push_back({TokenType::T_FLOAT, ""});
        } else if (substr == "bool") {
            tokens.push_back({TokenType::T_BOOL, ""});
        } else if (substr == "string") {
            tokens.push_back({TokenType::T_STRING, ""});
        } else if (substr == "\"") {
            tokens.push_back({TokenType::T_DOUBLE_QUOTE, ""});
            string_lit = !string_lit;
        } else if (substr == "(") {
            tokens.push_back({TokenType::T_ROUND_BRACKET_OPEN, ""});
        } else if (substr == ")") {
            tokens.push_back({TokenType::T_ROUND_BRACKET_CLOSE, ""});
        } else if (substr == "[") {
            tokens.push_back({TokenType::T_SQUARE_BRACKET_OPEN, ""});
        } else if (substr == "]") {
            tokens.push_back({TokenType::T_SQUARE_BRACKET_CLOSE, ""});
        } else if (substr == "{") {
            tokens.push_back({TokenType::T_CURLY_BRACKET_OPEN, ""});
        } else if (substr == "}") {
            tokens.push_back({TokenType::T_CURLY_BRACKET_CLOSE, ""});
        } else if (substr == ",") {
            tokens.push_back({TokenType::T_COMMA, ""});
        } else if (substr == ";") {
            tokens.push_back({TokenType::T_SEMICOLON, ""});
        } else if (substr == "=") {
            if (curr + 1 < trimmed_code.length() && trimmed_code.substr(curr, 2) == "==") {
                tokens.push_back({TokenType::T_EQUALS_OPR, ""});
                idx += 1;
            } else {
                tokens.push_back({TokenType::T_ASSIGNMENT_OPR, ""});
            }
        } else if (substr == "!") {
            if (curr + 1 < trimmed_code.length() && trimmed_code.substr(curr, 2) == "!=") {
                tokens.push_back({TokenType::T_NOT_EQUALS_OPR, ""});
                idx += 1;
            } else {
                tokens.push_back({TokenType::T_NOT, ""});
            }
        } else if (substr == "<") {
            if (curr + 1 < trimmed_code.length() && trimmed_code.substr(curr, 2) == "<=") {
                tokens.push_back({TokenType::T_LESS_THAN_EQUAL_TO_OPR, ""});
                idx += 1;
            } else if (curr + 1 < trimmed_code.length() && trimmed_code.substr(curr, 2) == "<<") {
                tokens.push_back({TokenType::T_LEFT_SHIFT_OPR, ""});
                idx += 1;
            } else {
                tokens.push_back({TokenType::T_LESS_THAN_OPR, ""});
            }
        } else if (substr == ">") {
            if (curr + 1 < trimmed_code.length() && trimmed_code.substr(curr, 2) == ">=") {
                tokens.push_back({TokenType::T_GREATER_THAN_EQUAL_TO_OPR, ""});
                idx += 1;
            } else if (curr + 1 < trimmed_code.length() && trimmed_code.substr(curr, 2) == ">>") {
                tokens.push_back({TokenType::T_RIGHT_SHIFT_OPR, ""});
                idx += 1;
            } else {
                tokens.push_back({TokenType::T_GREATER_THAN_OPR, ""});
            }
        } else if (substr == "&") {
            if (curr + 1 < trimmed_code.length() && trimmed_code.substr(curr, 2) == "&&") {
                tokens.push_back({TokenType::T_AND_OPR, ""});
                idx += 1;
            } else {
                tokens.push_back({TokenType::T_AND_OPR, ""});
            }
        } else if (substr == "|") {
            if (curr + 1 < trimmed_code.length() && trimmed_code.substr(curr, 2) == "||") {
                tokens.push_back({TokenType::T_OR_OPR, ""});
                idx += 1;
            } else {
                tokens.push_back({TokenType::T_OR_OPR, ""});
            }
        } else if (substr == "+") {
            tokens.push_back({TokenType::T_PLUS_OPR, ""});
        } else if (substr == "-") {
            tokens.push_back({TokenType::T_MINUS_OPR, ""});
        } else if (substr == "*") {
            tokens.push_back({TokenType::T_MULTIPLY_OPR, ""});
        } else if (substr == "/") {
            tokens.push_back({TokenType::T_DIVIDE_OPR, ""});
        } else if (string_lit) {
            tokens.push_back({TokenType::T_STRINGLIT, substr});
            string_lit = false;
            idx += 1;
        } else if (all_of(substr.begin(), substr.end(), ::isdigit)) {
            try {
                long long val = stoll(substr);
                tokens.push_back({TokenType::T_CONST_INT, to_string(val)});
            } catch (const out_of_range& oor) {
                cerr << "Out of Range error: " << oor.what() << endl;
            }
        } else if (try_parse_double(substr)) {
            double val = stod(substr);
            tokens.push_back({TokenType::T_CONST_FLOAT, to_string(val)});
        } else if (substr == "true" || substr == "false") {
            tokens.push_back({TokenType::T_CONST_BOOL, substr == "true" ? "true" : "false"});
        } else {
            // Identifier
            if (!substr.empty() && isdigit(substr[0])) {
                throw runtime_error("Identifiers should not start with numbers: " + substr);
            }
            tokens.push_back({TokenType::T_IDENTIFIER, substr});
        }

        curr = idx;
    }

    return tokens;
}

void print_tokens(const vector<Token>& tokens) {
    cout << "[";
    for (size_t i = 0; i < tokens.size(); ++i) {
        const Token& token = tokens[i];
        switch (token.type) {
            case TokenType::T_FUNCTION: cout << "T_FUNCTION"; break;
            case TokenType::T_IF: cout << "T_IF"; break;
            case TokenType::T_ELSE: cout << "T_ELSE"; break;
            case TokenType::T_ELSE_IF: cout << "T_ELSE_IF"; break;
            case TokenType::T_WHILE: cout << "T_WHILE"; break;
            case TokenType::T_FOR: cout << "T_FOR"; break;
            case TokenType::T_RETURN: cout << "T_RETURN"; break;
            case TokenType::T_PRINT: cout << "T_PRINT"; break;

            case TokenType::T_IDENTIFIER: cout << "T_IDENTIFIER(\"" << token.value << "\")"; break;
            case TokenType::T_STRINGLIT: cout << "T_STRINGLIT(\"" << token.value << "\")"; break;
            case TokenType::T_CONST_INT: cout << "T_CONST_INT(" << token.value << ")"; break;
            case TokenType::T_CONST_FLOAT: cout << "T_CONST_FLOAT(" << token.value << ")"; break;
            case TokenType::T_CONST_BOOL: cout << "T_CONST_BOOL(" << (token.value == "true" ? "true" : "false") << ")"; break;

            case TokenType::T_ROUND_BRACKET_OPEN: cout << "T_ROUND_BRACKET_OPEN"; break;
            case TokenType::T_ROUND_BRACKET_CLOSE: cout << "T_ROUND_BRACKET_CLOSE"; break;
            case TokenType::T_SQUARE_BRACKET_OPEN: cout << "T_SQUARE_BRACKET_OPEN"; break;
            case TokenType::T_SQUARE_BRACKET_CLOSE: cout << "T_SQUARE_BRACKET_CLOSE"; break;
            case TokenType::T_CURLY_BRACKET_OPEN: cout << "T_CURLY_BRACKET_OPEN"; break;
            case TokenType::T_CURLY_BRACKET_CLOSE: cout << "T_CURLY_BRACKET_CLOSE"; break;

            case TokenType::T_COMMA: cout << "T_COMMA"; break;
            case TokenType::T_SEMICOLON: cout << "T_SEMICOLON"; break;
            case TokenType::T_DOUBLE_QUOTE: cout << "T_DOUBLE_QUOTE"; break;

            case TokenType::T_ASSIGNMENT_OPR: cout << "T_ASSIGNMENT_OPR"; break;
            case TokenType::T_EQUALS_OPR: cout << "T_EQUALS_OPR"; break;
            case TokenType::T_NOT: cout << "T_NOT"; break;
            case TokenType::T_NOT_EQUALS_OPR: cout << "T_NOT_EQUALS_OPR"; break;
            case TokenType::T_LESS_THAN_OPR: cout << "T_LESS_THAN_OPR"; break;
            case TokenType::T_GREATER_THAN_OPR: cout << "T_GREATER_THAN_OPR"; break;
            case TokenType::T_LESS_THAN_EQUAL_TO_OPR: cout << "T_LESS_THAN_EQUAL_TO_OPR"; break;
            case TokenType::T_GREATER_THAN_EQUAL_TO_OPR: cout << "T_GREATER_THAN_EQUAL_TO_OPR"; break;
            case TokenType::T_AND_OPR: cout << "T_AND_OPR"; break;
            case TokenType::T_OR_OPR: cout << "T_OR_OPR"; break;
            case TokenType::T_RIGHT_SHIFT_OPR: cout << "T_RIGHT_SHIFT_OPR"; break;
            case TokenType::T_LEFT_SHIFT_OPR: cout << "T_LEFT_SHIFT_OPR"; break;

            case TokenType::T_PLUS_OPR: cout << "T_PLUS_OPR"; break;
            case TokenType::T_MINUS_OPR: cout << "T_MINUS_OPR"; break;
            case TokenType::T_MULTIPLY_OPR: cout << "T_MULTIPLY_OPR"; break;
            case TokenType::T_DIVIDE_OPR: cout << "T_DIVIDE_OPR"; break;

            case TokenType::T_INT: cout << "T_INT"; break;
            case TokenType::T_FLOAT: cout << "T_FLOAT"; break;
            case TokenType::T_BOOL: cout << "T_BOOL"; break;
            case TokenType::T_STRING: cout << "T_STRING"; break;

            default: throw runtime_error("Unknown token to print\n");
        }
        if (i < tokens.size() - 1) {
            cout << ", ";
        }
    }
    cout << "]" << endl;
}

int main() {
    string code = get_code();
    vector<Token> tokens = lex(code);

    print_tokens(tokens);

    return 0;
}