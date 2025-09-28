use crate::lexer::tokens::Token;

pub type RootList = Vec<Root>;

#[derive(Debug)]
pub enum Root{
    Var(VariableDeclaration),
    //Func(FunctionStatement),
}

#[derive(Debug)]
pub struct VariableDeclaration{ // for declaration of variables
    pub type_token: Token,
    pub identifier: String,
    pub expression: Expression,
}

#[derive(Debug)]
pub enum Expression{
    Literal(Constants),
    Identifier(String),
    BinaryOperation{ // like 5 + 3 // two expressions with an operator in between
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    UnaryOperation{ // like -5 or !abc // an operator followed by an expression
        operator: Token,
        expression: Box<Expression>,
    },
    // TODO
}

#[derive(Debug)]
pub enum Constants{
    Int(i64), // T_CONST_INT
    Float(f64), // T_CONST_FLOAT
    Str(String), // T_STRINGLIT
    Bool(bool), // T_CONST_BOOL
}