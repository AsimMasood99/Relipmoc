#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Token {
    T_FUNCTION, // fn
    T_IF, // if
    T_ELSE, // else
    T_ELSE_IF, // elif
    T_WHILE, // while
    T_FOR, // for
    T_RETURN, // return
    T_PRINT, // print

    T_IDENTIFIER(String),
    T_STRINGLIT(String),
    T_CONST_INT(i64),
    T_CONST_FLOAT(f64),
    T_CONST_BOOL(bool),

    T_ROUND_BRACKET_OPEN, // (
    T_ROUND_BRACKET_CLOSE, // )
    T_SQUARE_BRACKET_OPEN, // [
    T_SQUARE_BRACKET_CLOSE, // ]
    T_CURLY_BRACKET_OPEN, // {
    T_CURLY_BRACKET_CLOSE, // }

    T_COMMA, // ,
    //T_DOT, // .
    T_SEMICOLON, // ;
    T_DOUBLE_QUOTE, // "

    T_ASSIGNMENT_OPR, // =
    T_EQUALS_OPR, // ==
    T_NOT, // !
    T_NOT_EQUALS_OPR, // !=
    T_LESS_THAN_OPR, // <
    T_GREATER_THAN_OPR, // >
    T_LESS_THAN_EQUAL_TO_OPR, // <=
    T_GREATER_THAN_EQUAL_TO_OPR, // >=
    T_AND_OPR, // && , &
    T_OR_OPR, // || , |
    T_RIGHT_SHIFT_OPR, // >>
    T_LEFT_SHIFT_OPR, // <<

    T_INT, // 0
    T_FLOAT, // 0.0
    T_BOOL, // true | false
    T_STRING, // string
}