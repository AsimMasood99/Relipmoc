#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Token {
    T_FUNCTION, // fn

    T_IDENTIFIER(String),

    T_ROUND_BRACKET_OPEN, // (
    T_ROUND_BRACKET_CLOSE, // )
    T_SQUARE_BRACKET_OPEN, // [
    T_SQUARE_BRACKET_CLOSE, // ]
    T_CURLY_BRACKET_OPEN, // {
    T_CURLY_BRACKET_CLOSE, // }

    T_COMMA, // ,
    T_DOT, // .
    T_SEMICOLON, // ;
    T_DOUBLE_QUOTE, // "

    T_ASSIGNMENT_OPR, // =
    T_EQUALS_OPR, // ==

    T_INT, // 0
    T_FLOAT, // 0.0
    T_BOOL, // true | false
    T_STRING, // string
}

impl Token {
    pub fn sym(&self) -> String {
        match self {
            Token::T_FUNCTION => String::from("fn"),

            Token::T_IDENTIFIER(i) => i.clone(),

            Token::T_ROUND_BRACKET_OPEN => String::from("("),
            Token::T_ROUND_BRACKET_CLOSE => String::from(")"),
            Token::T_SQUARE_BRACKET_OPEN => String::from("["),
            Token::T_SQUARE_BRACKET_CLOSE => String::from("]"),
            Token::T_CURLY_BRACKET_OPEN => String::from("{"),
            Token::T_CURLY_BRACKET_CLOSE => String::from("}"),

            Token::T_COMMA => String::from(","),
            Token::T_DOT => String::from("."),
            Token::T_SEMICOLON => String::from(";"),
            Token::T_DOUBLE_QUOTE => String::from("\""),

            Token::T_ASSIGNMENT_OPR => String::from("="),
            Token::T_EQUALS_OPR => String::from("=="),
            
            Token::T_INT => String::from("int"),
            Token::T_FLOAT => String::from("float"),
            Token::T_BOOL => String::from("bool"),
            Token::T_STRING => String::from("string"),
        }
    }
}