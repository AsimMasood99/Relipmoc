# Relipmoc Lexer

A simple lexical analyzer (lexer) for a programming language, built as part of a compiler construction project.

## Features

- Tokenizes source code into meaningful tokens
- Supports various token types: numbers, strings, identifiers, keywords, operators, and delimiters
- Tracks line and column numbers for error reporting
- Extensible design for adding new token types

## Build Instructions

### Prerequisites
- CMake 3.16 or higher
- C++17 compatible compiler (GCC, Clang, or MSVC)

### Building the Project

```bash
# Create build directory
mkdir build
cd build

# Configure the project
cmake ..

# Build the project
make

# Run the lexer
./Relipmoc
```

### Building with Tests

```bash
# Configure with tests enabled
cmake -DBUILD_TESTS=ON ..

# Build everything including tests
make

# Run tests
ctest
# or run directly
./tests/test_lexer
```

## Project Structure

```
relipmoc/
├── CMakeLists.txt      # Main build configuration
├── README.md          # This file
├── src/               # Source files
│   ├── main.cpp       # Main entry point
│   ├── lexer.h        # Lexer class declaration
│   ├── lexer.cpp      # Lexer implementation
│   └── token.h        # Token definitions
├── tests/             # Unit tests
├── examples/          # Example input files
└── build/             # Build artifacts
```

## Usage

The lexer can be used to tokenize source code:

```cpp
#include "lexer.h"

std::string code = "int x = 42;";
Lexer lexer(code);
auto tokens = lexer.tokenize();

for (const auto& token : tokens) {
    std::cout << "Token: " << token.value << std::endl;
}
```

## Supported Tokens

- **Numbers**: Integer literals (e.g., `123`, `0`, `999`)
- **Identifiers**: Variable names (e.g., `variable`, `myVar`)
- **Keywords**: `if`, `else`, `while`, `for`, `return`
- **Operators**: `+`, `-`, `*`, `/`, `=`, `==`, `!=`, `<`, `>`
- **Delimiters**: `;`, `,`, `(`, `)`, `{`, `}`
- **Strings**: String literals (e.g., `"hello"`)

## Contributing

This is a learning project for compiler construction. Feel free to extend the lexer with additional features like:
- More operators and keywords
- Support for floating-point numbers
- Comments handling
- Error recovery mechanisms
