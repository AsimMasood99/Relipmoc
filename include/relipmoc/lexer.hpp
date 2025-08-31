#include "token.hpp"
#include <regex>
#include <string>
#include <vector>
#include <unordered_map>

class Lexer {
  private:
    const std::string &source;
    
    std::vector<std::pair<std::string, std::regex>> token_patterns;
    std::unordered_map<std::string, TokenType> keyword_map;
    std::unordered_map<std::string, TokenType> operator_map;
    
    void init_keyword_map();
    void init_operator_map();

  public:
    explicit Lexer(const std::string &src);
    std::vector<Token> tokenize();
};