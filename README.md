# Relipmoc 

Just an other compiler. 

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

## Usage

## Supported Tokens

- **Numbers**: Integer literals (e.g., `123`, `0`, `999`)
- **Identifiers**: Variable names (e.g., `variable`, `myVar`)
- **Keywords**: `if`, `else`, `while`, `for`, `return`
- **Operators**: `+`, `-`, `*`, `/`, `=`, `==`, `!=`, `<`, `>`
- **Delimiters**: `;`, `,`, `(`, `)`, `{`, `}`
- **Strings**: String literals (e.g., `"hello"`)