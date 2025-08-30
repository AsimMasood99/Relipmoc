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
            '!',
            '<',
            '>',
            '&',
            '|',
            '+',
            '-',
            '*',
            '/',
            '#',
        ].contains(&c)
}

fn lex(code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let code = code.trim().to_string();

    let mut curr = 0;
    let mut string_lit = false;

    while curr < code.len() {
        //println!("{}", &code[curr..]);

        // current ka baad spaced khatam karo
        if curr < code.len() {
            while code.chars().nth(curr).unwrap().is_whitespace() {
                curr += 1;
            }
        }

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
        //println!("Abhi wala: {substr}\n");

        if &code[curr..curr + 1] == "#" {
            if let Some(pos) = code[curr..].find('\n') {
                curr += pos + 1;
                continue;
            } else {
                break;
            }
        }

        if substr == "fn" {
            tokens.push(T_FUNCTION);
        }
        else if substr == "if" {
            tokens.push(T_IF);
        }
        else if substr == "else" {
            tokens.push(T_ELSE);
        }
        else if substr == "elif" {
            tokens.push(T_ELSE_IF);
        }
        else if substr == "while" {
            tokens.push(T_WHILE);
        }
        else if substr == "for" {
            tokens.push(T_FOR);
        }
        else if substr == "return" {
            tokens.push(T_RETURN);
        }
        else if substr == "print" {
            tokens.push(T_PRINT);
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
            if curr + 1 < code.len() && &code[curr..curr + 2] == "==" {
                tokens.push(T_EQUALS_OPR);
                idx = curr + 2;
            } else {
                tokens.push(T_ASSIGNMENT_OPR);
            }
        }
        else if substr == "!" {
            if curr + 1 < code.len() && &code[curr..curr + 2] == "!=" {
                tokens.push(T_NOT_EQUALS_OPR);
                idx = curr + 2;
            } else {
                tokens.push(T_NOT);
            }
        }
        else if substr == "<" {
            if curr + 1 < code.len() && &code[curr..curr + 2] == "<=" {
                tokens.push(T_LESS_THAN_EQUAL_TO_OPR);
                idx = curr + 2;
            } else if curr + 1 < code.len() && &code[curr..curr + 2] == "<<" {
                tokens.push(T_LEFT_SHIFT_OPR);
                idx = curr + 2;
            } else {
                tokens.push(T_LESS_THAN_OPR);
            }
        }
        else if substr == ">" {
            if curr + 1 < code.len() && &code[curr..curr + 2] == ">=" {
                tokens.push(T_GREATER_THAN_EQUAL_TO_OPR);
                idx = curr + 2;
            } else if curr + 1 < code.len() && &code[curr..curr + 2] == ">>" {
                tokens.push(T_RIGHT_SHIFT_OPR);
                idx = curr + 2;
            } else {
                tokens.push(T_GREATER_THAN_OPR);
            }
        }
        else if substr == "&" {
            if curr + 1 < code.len() && &code[curr..curr + 2] == "&&" {
                tokens.push(T_AND_OPR);
                idx = curr + 2;
            } else {
                tokens.push(T_AND_OPR); // single & treated as and too
            }
        }
        else if substr == "|" {
            if curr + 1 < code.len() && &code[curr..curr + 2] == "||" {
                tokens.push(T_OR_OPR);
                idx = curr + 2;
            } else {
                tokens.push(T_OR_OPR); // single | treated as OR too
            }
        }
        else if substr == "+" {
            tokens.push(T_PLUS_OPR);
        }
        else if substr == "-" {
            tokens.push(T_MINUS_OPR);
        }
        else if substr == "*" {
            tokens.push(T_MULTIPLY_OPR);
        }
        else if substr == "/" {
            tokens.push(T_DIVIDE_OPR);
        }
        else if string_lit {
            tokens.push(T_STRINGLIT(substr.clone()));
            string_lit = false;
            tokens.push(T_DOUBLE_QUOTE);
            idx += 1; // move past closing quote
        }
        else if substr.chars().all(|c| c.is_digit(10)) {
            let val: i64 = substr.parse().unwrap();
            tokens.push(T_CONST_INT(val));
        }
        else if substr.parse::<f64>().is_ok() {
            let val: f64 = substr.parse().unwrap();
            tokens.push(T_CONST_FLOAT(val));
        }
        else if substr == "true" || substr == "false" {
            tokens.push(T_CONST_BOOL(substr == "true"));
        }
        else {
            // not the job of lexer but requirment ma ikha ha...
            if substr.chars().next().unwrap().is_digit(10) {
                panic!("Identifiers should not start with numbers: {}", substr);
            }
            tokens.push(T_IDENTIFIER(substr.clone()));
        }
        curr = idx;
    }


    tokens
}

fn main() {
    let code = get_code();
    let tokens = lex(code);
    println!("{:?}", tokens);
}
