use std::fs;
use std::path::Path;

mod tokens;
use tokens::Token::{self, *};

fn get_code() -> String {
    // TODO: allow custom file path
    // let mut file_name = String::new();
    // print!("Enter the file name: ");
    // io::stdout().flush().unwrap(); // flush to print on screen
    // io::stdin().read_line(&mut file_name).unwrap();

    let path = Path::new("data").join("code.txt");
    let file_content = fs::read_to_string(path).unwrap();

    return file_content;
}

fn find_delim(c: char) -> bool {
    return c.is_whitespace() ||
        [
            '(',
            ')',
            '[',
            ']',
            '{',
            '}',
            ',',
            //'.',
            ';',
            '"',
            '=',
        ].contains(&c)
}

fn lex(code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let code = code.trim().to_string();

    let mut curr = 0;
    let mut string_lit = false;

    while curr < code.len() {
        println!("{}", &code[curr..]);

        let mut idx = if string_lit {
            let mut _idx = curr;
            loop {
                if let Some(pos) = code[_idx..].find('"') {
                    _idx += pos;
                    let mut backslash_count = 0;
                    let mut check_pos = _idx;
                    while check_pos > curr && code.chars().nth(check_pos - 1).unwrap() == '\\' {
                        backslash_count += 1;
                        check_pos -= 1;
                    }
                    // not escaped when back slash count is even
                    if backslash_count % 2 == 0 {
                        break;
                    }
                    _idx += 1;
                } else {
                    panic!("Unclosed string literal");
                }
            }
            _idx
        } else {
            let _idx = code[curr..].find(find_delim).unwrap() + curr;

            if _idx == curr {
                _idx + 1 // abhi tak delimeters 1 length k hain uper
            } else {
                _idx
            }
        };
        

        let substr = code[curr..idx].to_string();
        println!("Abhi wala: {substr}\n");

        if substr == "fn" {
            tokens.push(T_FUNCTION);
        }
        else if substr == "int" {
            tokens.push(T_INT);
        }
        else if substr == "float" {
            tokens.push(T_FLOAT);
        }
        else if substr == "bool" {
            tokens.push(T_BOOL);
        }
        else if substr == "string" {
            tokens.push(T_STRING);
        }
        else if substr == "\"" {
            tokens.push(T_DOUBLE_QUOTE);
            string_lit = !string_lit;
        }
        else if substr == "(" {
            tokens.push(T_ROUND_BRACKET_OPEN);
        }
        else if substr == ")" {
            tokens.push(T_ROUND_BRACKET_CLOSE);
        }
        else if substr == "[" {
            tokens.push(T_SQUARE_BRACKET_OPEN);
        }
        else if substr == "]" {
            tokens.push(T_SQUARE_BRACKET_CLOSE);
        } 
        else if substr == "{" {
            tokens.push(T_CURLY_BRACKET_OPEN);
        } 
        else if substr == "}" {
            tokens.push(T_CURLY_BRACKET_CLOSE);
        } 
        else if substr == "," {
            tokens.push(T_COMMA);
        } 
        // else if substr == "." {
        //     tokens.push(T_DOT);
        // } 
        else if substr == ";" {
            tokens.push(T_SEMICOLON);
        }
        else if substr == "=" {
            tokens.push(T_ASSIGNMENT_OPR);
        }
        else if substr == "==" {
            tokens.push(T_EQUALS_OPR);
        }
        else if string_lit {
            tokens.push(T_STRINGLIT(substr.clone()));
            string_lit = false;
            tokens.push(T_DOUBLE_QUOTE);
            idx += 1; // move past closing quote
        }
        else {
            tokens.push(T_IDENTIFIER(substr.clone()));
        }
        curr = idx;
        if curr < code.len() {
            while code.chars().nth(curr).unwrap().is_whitespace() {
                curr += 1;
            }
        }
    }


    tokens
}

fn main() {
    let code = get_code();
    let tokens = lex(code);
    println!("{:?}", tokens);
}
