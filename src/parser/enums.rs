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

#[derive(Debug)]
pub struct Parameter{
    pub param_type: Token,
    pub identifier: String,
}

#[derive(Debug)]
pub struct FunctionStatement{
    pub return_type: Token,
    pub identifier: String,
    pub parameters: Vec<Parameter>,
    pub block: Block,
}

#[derive(Debug)]
pub struct Block{
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement{
    VarDecl(VariableDeclaration),
    Expr(Expression),
    //Return(Expression),
    If(IfStatement),
    // While(WhileStatement),
    For(ForStatement),
    // Function(FunctionStatement),
}

#[derive(Debug)]
pub struct IfStatement{
    pub condition: Expression,
    pub block: Block,
    pub elif_blocks: Vec<ElifBlock>,
    pub else_block: Option<Block>,
}

#[derive(Debug)]
pub struct ElifBlock{
    pub condition: Expression,
    pub block: Block,
}

#[derive(Debug)]
pub struct ForStatement {
    pub init_var: Option<VariableDeclaration>,
    pub condition: Option<Expression>,
    pub update: Option<Expression>,
    pub block: Block,
}