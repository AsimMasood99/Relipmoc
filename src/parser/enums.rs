use crate::lexer::tokens::Token;

pub type RootList = Vec<Root>;

#[derive(Debug)]
pub enum Root{
    Var(VariableDeclaration),
    Func(FunctionStatement),
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
    // TODO
}

#[derive(Debug)]
pub enum Constants{
    Int(i64), // T_CONST_INT
    Float(f64), // T_CONST_FLOAT
    Str(String), // T_STRINGLIT
    Bool(bool), // T_CONST_BOOL
}

#[derive(Debug)]
pub struct Parameter{
    pub param_type: Token,
    pub identifier: String,
}

#[derive(Debug)]
pub struct Block{
}

#[derive(Debug)]
pub struct FunctionStatement{
    pub return_type: Token,
    pub identifier: String,
    pub parameters: Vec<Parameter>,
    pub block: Block,
}