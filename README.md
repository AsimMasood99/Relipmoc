# Relipmoc

Not just your avarage compiler. 

## Language Syntax Support

- Function definitions (`fn`)
- Control flow (`if`, `else`, `elif`, `while`, `for`)
- Data types (`int`, `float`, `bool`, `string`)
- Operators (arithmetic, comparison, logical, bitwise)
- Comments (line comments with `#`)
- String literals with escape sequences

## Build Instructions

### Prerequisites
- CMake 3.5 or higher
- C++17 compatible compiler (GCC, Clang, or MSVC)

### Building the Project

```bash
mkdir build
cd build

cmake ..

make

./Relipmoc
```

### Building with Tests

```bash
TO BE DEFINED LATER.
```

## Project Structure

```
relipmoc/
├── CMakeLists.txt              # Main build configuration
├── README.md                   # This file
├── include/                    # Header files
│   └── relipmoc/
│       ├── lexer.hpp          # Lexer class declaration
│       └── token.hpp          # Token definitions and enums
├── src/                       # Source files
│   ├── main.cpp              # Main entry point with demo
│   ├── lexer.cpp             # Regex-based lexer implementation
│   └── token.cpp             # Token utility functions
├── tests/                     # Unit tests
│   ├── CMakeLists.txt        # Test build configuration
│   └── test_lexer.cpp        # Basic test cases
├── examples/                  # Example input files
│   └── sample.txt            # Sample code for testing
└── build/                    # Build artifacts
```

## Usage

See /src/main.cpp

## Supported Tokens

### Keywords
- **Control Flow**: `if`, `else`, `elif`, `while`, `for`, `return`
- **Functions**: `fn`, `print`
- **Data Types**: `int`, `float`, `bool`, `string`

### Literals
- **Integers**: `123`, `0`, `999`
- **Floats**: `3.14`, `0.5`, `123.456`
- **Strings**: `"hello"`, `"world with \"quotes\""`, `"escaped\\text"`
- **Identifiers**: `variable`, `myVar`, `_private`, `count2`

### Operators
- **Arithmetic**: `+`, `-`, `*`, `/`
- **Assignment**: `=`
- **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Logical**: `&&`, `||`, `!`
- **Increment/Decrement**: `++`, `--`
- **Bitwise**: `<<`, `>>` (can be added to regex)

### Delimiters and Punctuation
- **Parentheses**: `(`, `)`
- **Brackets**: `[`, `]`
- **Braces**: `{`, `}`
- **Separators**: `;`, `,`, `.`

### Special
- **Comments**: `# This is a comment`
- **Whitespace**: Spaces, tabs, newlines (automatically filtered)

## TODO: 
Define TODO list here. 