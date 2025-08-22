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
            T_ROUND_BRACKET_OPEN.sym(),
            T_ROUND_BRACKET_CLOSE.sym(),
            T_SQUARE_BRACKET_OPEN.sym(),
            T_SQUARE_BRACKET_CLOSE.sym(),
            T_CURLY_BRACKET_OPEN.sym(),
            T_CURLY_BRACKET_CLOSE.sym(),
            T_COMMA.sym(),
            T_DOT.sym(),
            T_SEMICOLON.sym(),
            T_DOUBLE_QUOTE.sym(),
            T_ASSIGNMENT_OPR.sym(),
            T_EQUALS_OPR.sym(),
        ].contains(&c.to_string())
}

fn lex(code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let code = code.trim().to_string();

    let mut curr = 0;
    let mut string_lit = false;

    while curr < code.len() {
        println!("{}", &code[curr..]);

        let mut idx = if string_lit {
            let mut _idx = code[curr..].find(&T_DOUBLE_QUOTE.sym()).unwrap() + curr;
            while code.chars().nth(_idx - 1).unwrap() == '\\' {
                _idx = code[(_idx+1)..].find(&T_DOUBLE_QUOTE.sym()).unwrap() + _idx + 1;
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

        if substr == T_FUNCTION.sym() {
            tokens.push(T_FUNCTION);
        }
        else if substr == T_INT.sym() {
            tokens.push(T_INT);
        }
        else if substr == T_FLOAT.sym() {
            tokens.push(T_FLOAT);
        }
        else if substr == T_BOOL.sym() {
            tokens.push(T_BOOL);
        }
        else if substr == T_STRING.sym() {
            tokens.push(T_STRING);
        }
        else if substr == T_DOUBLE_QUOTE.sym() {
            tokens.push(T_DOUBLE_QUOTE);
            string_lit = !string_lit;
        }
        else if substr == T_ROUND_BRACKET_OPEN.sym() {
            tokens.push(T_ROUND_BRACKET_OPEN);
        }
        else if substr == T_ROUND_BRACKET_CLOSE.sym() {
            tokens.push(T_ROUND_BRACKET_CLOSE);
        }
        else if substr == T_SQUARE_BRACKET_OPEN.sym() {
            tokens.push(T_SQUARE_BRACKET_OPEN);
        }
        else if substr == T_SQUARE_BRACKET_CLOSE.sym() {
            tokens.push(T_SQUARE_BRACKET_CLOSE);
        } 
        else if substr == T_CURLY_BRACKET_OPEN.sym() {
            tokens.push(T_CURLY_BRACKET_OPEN);
        } 
        else if substr == T_CURLY_BRACKET_CLOSE.sym() {
            tokens.push(T_CURLY_BRACKET_CLOSE);
        } 
        else if substr == T_COMMA.sym() {
            tokens.push(T_COMMA);
        } 
        else if substr == T_DOT.sym() {
            tokens.push(T_DOT);
        } 
        else if substr == T_SEMICOLON.sym() {
            tokens.push(T_SEMICOLON);
        }
        else if substr == T_ASSIGNMENT_OPR.sym() {
            tokens.push(T_ASSIGNMENT_OPR);
        }
        else if substr == T_EQUALS_OPR.sym() {
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
