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

    // Trim leading and trailing whitespace
    size_t first = trimmed_code.find_first_not_of(" \t\n\r");
    if (string::npos == first) {
        return tokens; // Empty string
    }
    size_t last = trimmed_code.find_last_not_of(" \t\n\r");
    trimmed_code = trimmed_code.substr(first, (last - first + 1));

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
                if (trimmed_code[_idx] == '"') {
                    size_t backslash_count = 0;
                    size_t check_pos = _idx;
                    while (check_pos > curr && trimmed_code[check_pos - 1] == '\\') {
                        backslash_count++;
                        check_pos--;
                    }
                    // Not escaped when backslash count is even
                    if (backslash_count % 2 == 0) {
                        idx = _idx;
                        break;
                    }
                    _idx++;
                } else {
                    throw runtime_error("Unclosed string literal");
                } 
            }
            if (idx == string::npos) {
                throw runtime_error("Unclosed string literal");
            }
        } else {
            size_t found = string::npos;
            for (char delim : string("(){}[]\" ,;=!<>&|+*-/")) {
                size_t pos = trimmed_code.find(delim, curr);
                if (pos != string::npos && (found == string::npos || pos < found)) {
                    found = pos; // keep the earliest occurrence
                }
            }

            if (found == string::npos) {
                idx = trimmed_code.length();
            } else {
                idx = found + curr;
            }
            if (idx == curr) {
                idx++; // Delimiters are 1 length
            }
        }

        string substr = trimmed_code.substr(curr, idx - curr);

        if (substr == "#") {
            size_t newline_pos = trimmed_code.find('\n', curr);
            if (newline_pos != string::npos) {
                curr = newline_pos + 1;
                continue;
            } else {
                break;
            }
        }

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

int main() {
    string code = get_code();
    vector<Token> tokens = lex(code);

    for (const auto& token : tokens) {
        cout << "Type: " << static_cast<int>(token.type) << ", Value: " << token.value << endl;
    }

    return 0;
}