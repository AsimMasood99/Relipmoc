
    // defining some aliases
pub type root_list = Vec<root>;
// pub type Type = Token; // {Int,String,Float,Bool}
// this Token will be defined in the lexer and used here as type so we dont need to define type again


#[derive(Debug)]
pub enum root{
    Var(variable_declaration),
    Func(function_statement),
}

#[derive(Debug)]
pub struct variable_declaration{ // for declaration of variables
    pub type_token: Type,
    pub identifier: String,
    pub expression: Expr,
}

#[derive(Debug)]
pub struct function_statement{
    pub type_token: Type,
    pub identifier: String,
    // left curly braces
    pub params: Vec<param>, // we are handeling params as vector of params
    // right curly braces
    pub block: Block,
}


// #[derive(Debug)]
// pub enum function_type{
//     pub type_token: Type,
// }

// RENOTE: params are being handeled as vector of strings for now (inside function_statement structs)

#[derive(Debug)]
pub struct param{
    pub type_token: Type,
    pub identifier: String,
}

#[derive(Debug)]
pub struct Block{
    // left curly braces
    pub statements: Vec<statement>, // "statements . statements"
    // right curly braces
}

// language of statements are handeled in the Block struct

#[derive(Debug)]
pub enum statement{
    If(if_statement),
    // While(while_statement),
    For(for_statement),
    Var(variable_declaration),
    Expr(expression_statement),
    Break,
    Continue,
    Return(return_statement),
    Block(Block), // nested block
}

#[derive(Debug)]
pub struct for_statement{
    // T_FOR,
    // left parenthesis
    pub init_loop_var: Option<variable_declaration>, // Option enum allows absence of values
    pub loop_condition: expression_statement,
    pub update_loop_var: Option<expression>, // just a simple number as expression (j++, ++j)
    pub block:Block
}

#[derive(Debug)]
pub enum init_loop_var{
    Var(variable_declaration),
    // semi colon // TODO: check is this approach correct as semi colon is a  
}

// update-loop-var taken care of in the for_statement struct

#[derive(Debug)]
pub struct if_statement{
    // T_IF,
    pub if_statement: if_statement_expression,
    pub elif_statement: Vec<elif_statement>,
    //else 
    pub else_statement: Option<Block>, 
}

#[derive(Debug)]
pub struct if_statement_expression{
    // if
    // left round bracket
    pub condition: Expr,
    // right round bracket
    block: Block
}

#[derive(Debug)]
pub enum elif_statement{
    Elif{
        // T_ELSE_IF,
        if_statement: if_statement_expression,
        elif_statement: Option<Box<elif_statement>>, // To allocate memory on heap for recursive elif statements
        // right curly bracket
    }
}

// else-statement taken care of in the if_statement struct

#[derive(Debug)]
pub struct return_statement{
    // T_RETURN,
    pub expr: expression_statement,
}

// break and continue dont need separate structs

#[derive(Debug)]
pub enum expression_statement{
    Expr(Expr),
    SemiColon, // to handle empty expression statements
}

#[derive(Debug)]
pub enum assignment_expression{
        Boolean(boolean_expression),
        assigning(
            boolean_expression,
            // assignment_operator // =,+=,-=,*=,/=,%= (terminal in lexer)
            Box<assignment_expression> // recursive to allow chaining of assignments
        )
}

#[derive(Debug)]
pub enum boolean_expression{
    BitwiseOr(Bit_or_expression),
    Or(
        Bit_or_expression,
        // T_OR, // terminal in lexer
        Box<boolean_expression> // recursive to allow chaining of 'or' operations
    )
}

# [derive(Debug)]
pub enum Bit_or_expression{
    BitwiseAnd(Bit_and_expression),
    BitOr(
        Bit_and_expression,
        // T_AND, // terminal in lexer
        Box<Bit_or_expression> // recursive to allow chaining of 'and' operations
    )
}

# [derive(Debug)]
pub enum Bit_and_expression{
    compare(compare_expression),
    BitAnd(
        compare_expression,
        // comparison_operator, // <,>,<=,>=,==,!= (terminal in lexer)
        Box<Bit_and_expression> // recursive to allow chaining of comparison operations
    )
}


# [derive[Debug]]
pub enum compare_expression{
    shift(shift_expression),
    Compare(
        shift_expression,
        comparison_operator, // <,>,<=,>=,==,!= (terminal in lexer)
        Box<compare_expression> // recursive to allow chaining of comparison operations
    )
}

# [derive(Debug)]
pub enum shift_expression{
    addition(addition_expression),
    Shift(
        addition_expression,
        shift_operator, // <<,>>   (terminal in lexer)
        Box<shift_expression> // recursive to allow chaining of shift operations
    )
} 

# [derive(Debug)]   
pub enum addition_expression{
    multiplication(multiplication_expression),
    Add(
        multiplication_expression,
        addition_operator, // +,-   (terminal in lexer)
        Box<addition_expression> // recursive to allow chaining of addition operations
    )
}

# [derive(Debug)]
pub enum multiplication_expression{
    unary(unary_expression),
    Multiply(
        unary_expression,
        multiplication_operator, // *,/,%,   (terminal in lexer)
        Box<multiplication_expression> // recursive to allow chaining of multiplication operations
    )
}

# [derive(Debug, Clone)]
pub enum multiplication_operator {
    Multiply,  // *
    Divide,    // /
    Modulo,    // %
}

# [derive(Debug)]
pub enum Exp_expression{
    unary(unary_expression),
    Exp(
        unary_expression,
        exp_operator, // **   (terminal in lexer)
        Box<Exp_expression> // recursive to allow chaining of exp operations
    )
}


# [derive(Debug)]
pub enum unary_expression{
    primary(primary_expression),
    UnaryOp(
        unary_operator, // !,~,++,--,+,-   (terminal in lexer)
        Box<unary_expression> // recursive to allow chaining of unary operations
    )
}


# [derive(Debug, Clone)]
pub enum unary_operator {
    Minus,  // -
    Not,    // !
}


// TODO: is this the primary_expression correct

#[derive(Debug)]
pub enum primary_expression{
    Literal(Literal), // int, float, string, bool
    Identifier(String), // variable name
    FuncCall(function_call),
    // (expression)
    Expr(Box<Expr>), // Box to allocate memory on heap for recursive expressions
    funcCall(function_call),
}

// #[derive(Debug)]
// pub enum primary_expression {
//     ConstInt(i64),      // T_CONST_INT
//     ConstFloat(f64),    // T_CONST_FLOAT 
//     StringLit(String),  // T_STRINGLIT
//     BoolLit(bool),      // bool-literal
//     Identifier(String), // T_IDENTIFIER
//     ParenExpr(Box<Expr>), // T_ROUND_BRACKET_OPEN expr T_ROUND_BRACKET_CLOSE
//     FuncCall(function_call) // function-call
// }


// bool-literal

#[derive(Debug)]
pub enum bool_literal {
    True,
    False,
}

#[derive(Debug)]
pub struct function_call{
    pub identifier: String, // function name
    // left round bracket
    pub args: Vec<Expr>, // Vec handles zero, one, or many arguments
    // right round bracket
}
