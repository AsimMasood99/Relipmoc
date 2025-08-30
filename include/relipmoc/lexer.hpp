#include "token.hpp"
#include <regex>
#include <string>
#include <vector>

class Lexer {
private:
  const std::string &source;
  std::vector<std::pair<std::string, std::regex>> token_patterns;

public:
  explicit Lexer(const std::string &src);
  std::vector<Token> tokenize();
};