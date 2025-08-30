#include "../include/relipmoc/lexer.hpp"
#include <iostream>
Lexer::Lexer(const std::string &src) : source(src) {
  this->token_patterns = {
      {"Keyword", std::regex("\\b(fn|if|else|elif|while|for|return|print|int|"
                             "float|bool|string)\\b")},
      {"Identifier", std::regex("[a-zA-Z_][a-zA-Z0-9_]*")},
      {"Number", std::regex("[0-9]+")},
      {"Operator", std::regex(R"(==|!=|<=|>=|\+\+|--|&&|\|\||[=+\-*/<>!&|])")},
      {"Semicolon", std::regex(";")},
      {"Whitespace", std::regex("\\s+")} // we'll skip this later
  };
}

std::vector<Token> Lexer::tokenize() {
  std::smatch match;
  std::vector<Token> tokens;

  for (const auto &[name, pattern] : token_patterns) {
    if (std::regex_search(source, match, pattern)) {
      std::cout << "Found " << name << ": " << match.str() << std::endl;
      //   tokens.emplace_back(name, match.str());
    }
  }

  return tokens;
}
