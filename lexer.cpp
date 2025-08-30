#include <iostream>
#include <unordered_map>
#include <string>
#include <fstream>


//Other than below tokens T_indentifier("string")
//T_CONST_NUM_VALUE(Value)

std::unordered_map<std::string, std::string> tokenizationMap = 
{
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



//Utility Functions
bool isDelim(const char& character)
{
    return (character == ' ' || character == '\n');
}

bool isNum(const std::string& bufferString)
{
   
    bool hasDecimal = false;
    
    int startIdx = 0;
    if (bufferString[0] == '-')
    {
        startIdx = 1;
    }
    
    // Edge case minus sign should not be just a string
    if (startIdx == 1 && bufferString.length() == 1)
    {
        return false;
    }
    
    for (int i = startIdx; i < bufferString.length(); i++) 
    {
        char c = bufferString[i];
        
        if (c == '.') 
        {
            if (hasDecimal)
            {
                return false;
            }

            hasDecimal = true;
        }
        else if (!isdigit(c)) 
        {
            return false;
        }
    }
    
    return true;
}


//The Big One
void convertStringAndWriteToFile(const std::string& bufferString, std::ofstream& outputFile, const bool isStringFlag)
{
    if (isStringFlag)
    {
        outputFile << "T_STRING(\"" << bufferString << "\")," << " ";
        return;
    }

    //If is Num save token as T_CONST_NUM_VALUE
    if (isNum(bufferString))
    {
        outputFile << "T_CONST_NUM_VALUE(" << bufferString << ")," << " ";
        return;
    }
    
    //See if is a keyword or an operator
    auto it = tokenizationMap.find(bufferString);

    //it's a keyword/operator
    if (it != tokenizationMap.end())
    {
        outputFile << it->second << "," << " ";

    }
    else //it is an identifier
    {
        outputFile << "T_IDENTIFIER(\"" << bufferString << "\")," << " ";
    }
}

void lex(const std::string& codeFile)
{
    std::ifstream file(codeFile);
    if (!file.is_open())
    {
        std::cerr << "Error: Could not open file " << codeFile << std::endl;
        return;
    }

    std::ofstream outputFile("tokensOutput.txt");
    if (!outputFile.is_open())
    {
        std::cerr << "Error: Could not open output file." << std::endl;
        return;
    }

    std::string currentBuffer;
    char currCharacter;
    while (file.get(currCharacter))
    {
        if (isDelim(currCharacter));
        {
            //TODO: handle is string case
            convertStringAndWriteToFile(currentBuffer, outputFile, false);
            currentBuffer.clear();
        }
    }

    file.close();
}