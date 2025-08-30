#include "../include/relipmoc/lexer.hpp"
#include "../include/relipmoc/token.hpp"
#include <iostream>

int main() {
  std::string code = R"(
        fn main() {
            let x = 42;
            if (x > 0) {
                print("Positive");
            } else {
                print("Non-positive");
            }
        }
    )";

  Lexer lexer(code);
  std::vector<Token> tokens = lexer.tokenize();
  return 0;
}
