#include <iostream>
#include <unordered_map>
#include <string>
#include <fstream>


//Other than below tokens T_indentifier("string")
//T_CONST_NUM_INT(Value)
//T_CONST_NUM_FLOAT(Value)

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
    {"++", "T_INCREMENT"},
    {"--", "T_DECREMENT"},
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
    {"\"", "T_DOUBLE_QUOTE"},
    {"'", "T_SINGLE_QUOTE"}
};

//Utility Functions
bool isDelim(const char& character)
{
    return (character == ' ' || character == '\n');
}

bool isNum(const std::string& bufferString, bool& floatFlag)
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
    
    floatFlag = hasDecimal;

    return true;
}
bool isInvalidIdentifier(const std::string& bufferString)
{

    if (isdigit(bufferString[0]))
    {
        std::cout<<bufferString<< " is an invalid identifier as it starts with a digit" <<std::endl;
        return true;
    }

    for (char c : bufferString)
    {
        if (!isalnum(c) && c != '_')
        {
            std::cout<<bufferString<< " is an invalid identifier as it contains invalid characters" <<std::endl;
            return true;
        }
    }

    return false;
}
bool isOperator(const char& character)
{
    std::string operators = "+-*/%<>=!&|^~";
    return operators.find(character) != std::string::npos;
}
bool isDoubleOperator(const char& firstChar, const char& secondChar)
{
    std::string doubleOperators[] = {"<=", ">=", "==", "!=", "&&", "||", "++", "--", "<<", ">>"};
    std::string combined;
    combined += firstChar;
    combined += secondChar;

    for (auto itr = std::begin(doubleOperators); itr != std::end(doubleOperators); ++itr)
    {
        if (combined == *itr)
        {
            return true;
        }
    }
    return false;
}


//The Big One
void convertStringAndWriteToFile(const std::string& bufferString, std::ofstream& outputFile, const bool isStringFlag)
{
    bool isFloatFlag = false;

    if (isStringFlag)
    {
        outputFile <<tokenizationMap["\""] <<", "<< "T_STRING(\"" << bufferString << "\"), " << tokenizationMap["\""] <<", ";
        return;
    }

    //If is Num save token as T_CONST_NUM_Type
    if (isNum(bufferString, isFloatFlag))
    {
        if (isFloatFlag)
        {
            outputFile << "T_CONST_NUM_FLOAT(" << bufferString << "), ";
        }
        else
        {
            outputFile << "T_CONST_NUM_INT(" << bufferString << "), ";
        }
        return;
    }
    
    //See if is a keyword or an operator
    auto it = tokenizationMap.find(bufferString);

    //it's a keyword/operator
    if (it != tokenizationMap.end())
    {
        outputFile << it->second << ", ";

    }
    else //it is an identifier
    {
        if(isInvalidIdentifier(bufferString))
        {
            std::cerr << "Error: Invalid identifier encountered: " << bufferString << std::endl;
            return;
        }
    }
}


// TODO: check for ==, <= double operators check
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
    bool stringStarted = false;
    bool escapeCharNext = false;

    while (file.get(currCharacter))
    {
        if(stringStarted)
        {
            //have seen escape character for the first time
            if (currCharacter == '\\' && !escapeCharNext)
            {
                escapeCharNext = true;
                currentBuffer+= currCharacter;
                continue;
            }

            //Character was escaped
            if (escapeCharNext)
            {
                currentBuffer += currCharacter;
                escapeCharNext = false;
                continue;
            }

            //Exiting String
            if (currCharacter == '\"')
            {
                currentBuffer += currCharacter;
                stringStarted = false;
                convertStringAndWriteToFile(currentBuffer, outputFile, true);
                currentBuffer.clear();
                continue;
            }

            // Normal string characters
            currentBuffer += currCharacter;
        }
        else // Normal Case
        {
            //Operators case
            if(isOperator(currCharacter) && currentBuffer.empty()) //just read a new operator from start
            {
                currentBuffer += currCharacter;
                //PEAK NEXT TO SEE IF TWO OPERATOR ONE
                char nextCharacter = file.peek();
                if (file.eof() || !isOperator(nextCharacter))
                {
                    convertStringAndWriteToFile(currentBuffer, outputFile, false);
                    currentBuffer.clear();
                }
                else
                {
                    if(isDoubleOperator(currCharacter, nextCharacter))
                    {
                        currentBuffer += nextCharacter;
                        file.get();
                        convertStringAndWriteToFile(currentBuffer, outputFile, false);
                        currentBuffer.clear();
                    }
                }
                continue;
            }
            if(currCharacter == '\"')
            {
                if (!currentBuffer.empty()) //string starting so clear buffer
                {
                    convertStringAndWriteToFile(currentBuffer, outputFile, false);
                    currentBuffer.clear();
                }
                stringStarted = true;
                currentBuffer += currCharacter;
                continue;
            }
            else if (isDelim(currCharacter))
            {
                if(!currentBuffer.empty()) // if not multiple spaces
                {
                    convertStringAndWriteToFile(currentBuffer, outputFile, false);
                    currentBuffer.clear();
                }
            }
            else
            {
                currentBuffer+= currCharacter;
            }
        }
    }

    //if buffer still left, TODO : remove this debug
    if (!currentBuffer.empty())
    {
        std::cout<<"Buffer still left: "<<currentBuffer<<std::endl;
        convertStringAndWriteToFile(currentBuffer, outputFile, false);
        currentBuffer.clear();
    }

    file.close();
    outputFile.close();
}