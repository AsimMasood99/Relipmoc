use crate::parser::enums::{*};
use crate::lexer::tokens::Token;
use crate::parser::token_iterator::TokenIterator;
use crate::parser::errors::Errors;

// TODO: handle rest of non constants expressions
fn parse_expression(tokens: &mut TokenIterator) -> Result<Expression, Errors> {
    let current = tokens.consume()?;

    match current {
        Token::T_CONST_INT(value) => {
            Ok(Expression::Literal(Constants::Int(value.clone())))
        }
        Token::T_CONST_FLOAT(value) => {
            Ok(Expression::Literal(Constants::Float(value.clone())))
        }
        Token::T_DOUBLE_QUOTE => {
            let value = match tokens.consume()? {
                Token::T_STRINGLIT(s) => s.clone(),
                _ => return Err(Errors::ExpectedStringLit),
            };
            tokens.seek_if(Token::T_DOUBLE_QUOTE)?;
            Ok(Expression::Literal(Constants::Str(value.clone())))
        }
        Token::T_CONST_BOOL(value) => {
            Ok(Expression::Literal(Constants::Bool(value.clone())))
        }
        other => Err(Errors::UnexpectedToken(other.clone())),
    }
}

fn parse_variable_declaration(tokens: &mut TokenIterator) -> Result<VariableDeclaration, Errors> {

    // type like int, float, bool, string
    let var_type = match tokens.consume()? {
        Token::T_INT => Token::T_INT,
        Token::T_STRING => Token::T_STRING,
        Token::T_FLOAT => Token::T_FLOAT,
        Token::T_BOOL => Token::T_BOOL,
        other => return Err(Errors::ExpectedTypeToken(other.clone())),
    };

    // identifier the name of the variable
    let var_identifier = match tokens.consume()? {
        Token::T_IDENTIFIER(name) => name.clone(), // matching and if it matched copy (clone) the name
        other => return Err(Errors::ExpectedIdentifier(other.clone())) // no match: send error
    };

    // '=' token
    tokens.seek_if(Token::T_ASSIGNMENT_OPR)?;

    let expression = parse_expression(tokens)?;

    // ';' token
    tokens.seek_if(Token::T_SEMICOLON)?;

    Ok(VariableDeclaration {
        type_token: var_type,
        identifier: var_identifier,
        expression: expression,
    })
}

fn parse_parameter(tokens: &mut TokenIterator) -> Result<Parameter, Errors> {

    let param_type = match tokens.consume()? {
        Token::T_INT => Token::T_INT,
        Token::T_STRING => Token::T_STRING,
        Token::T_FLOAT => Token::T_FLOAT,
        Token::T_BOOL => Token::T_BOOL,
        other => return Err(Errors::ExpectedTypeToken(other.clone())),
    };

    // identifier the name of the variable
    let param_identifier = match tokens.consume()? {
        Token::T_IDENTIFIER(name) => name.clone(), 
        other => return Err(Errors::ExpectedIdentifier(other.clone()))
    };

    Ok(Parameter {
        param_type: param_type,
        identifier: param_identifier,
    })
}

fn parse_parameter_list(tokens: &mut TokenIterator) -> Result<Vec<Parameter>, Errors> {
    let mut parameters = Vec::new();

    // if next token is ')', then no parameters
    if let Some(Token::T_ROUND_BRACKET_CLOSE) = tokens.peek_curr() {
        return Ok(parameters);
    }

    loop {
        let param = parse_parameter(tokens)?;
        parameters.push(param);

        if let Some(Token::T_COMMA) = tokens.peek_curr() {
            tokens.consume()?; // consume the comma
            continue;
        } else {
            break; // no more parameters
        }
    }

    Ok(parameters)
}

fn parse_function_statement(tokens: &mut TokenIterator) -> Result<FunctionStatement, Errors> {

    tokens.consume().unwrap(); // consume 'func' token
    let (return_type, func_name) = match tokens.peek_curr() {
        Some(Token::T_INT) | Some(Token::T_STRING) | Some(Token::T_FLOAT) | Some(Token::T_BOOL) => {
            // Explicit return type provided
            let ret_type = match tokens.consume()? {
                Token::T_INT => Token::T_INT,
                Token::T_STRING => Token::T_STRING,
                Token::T_FLOAT => Token::T_FLOAT,
                Token::T_BOOL => Token::T_BOOL,
                other => return Err(Errors::ExpectedTypeToken(other.clone())),
            };
            
            // function name
            let name = match tokens.consume()? {
                Token::T_IDENTIFIER(name) => name.clone(),
                other => return Err(Errors::ExpectedIdentifier(other.clone())),
            };
            
            (ret_type, name)
        }
        Some(Token::T_IDENTIFIER(_)) => {
            let name = match tokens.consume()? {
                Token::T_IDENTIFIER(name) => name.clone(),
                other => return Err(Errors::ExpectedIdentifier(other.clone())),
            };
            
            (Token::T_VOID, name) // Default to void
        }
        other => {
            // Handle the Option properly
            match other {
                Some(token) => return Err(Errors::ExpectedIdentifier(token.clone())),
                None => return Err(Errors::UnexpectedEOF),
            }
        }
    };

    // opening parenthesis
    tokens.seek_if(Token::T_ROUND_BRACKET_OPEN)?;

    // parameters
    let parameters = parse_parameter_list(tokens)?;

    // closing parenthesis
    tokens.seek_if(Token::T_ROUND_BRACKET_CLOSE)?;

    Ok(FunctionStatement {
        return_type,
        identifier: func_name,
        parameters,
        block: Block{}
    })
}

// fn parse_block(tokens: &mut TokenIterator) -> Result<Block, Errors> {
// }


pub fn parser(tokens: Vec<Token>) -> RootList {
    let mut token_iterator = TokenIterator::new(tokens);
    let mut roots: RootList = vec![];

    while !token_iterator.is_at_end() {
        let current = token_iterator.peek_curr();
        match current {
            Some(Token::T_INT) | Some(Token::T_FLOAT) | Some(Token::T_BOOL) | Some(Token::T_STRING) => {
                match parse_variable_declaration(&mut token_iterator) {
                    Ok(var_decl) => roots.push(Root::Var(var_decl)),
                    Err(e) => {
                        panic!("Error parsing variable declaration: {:?}", e);
                    }
                }
            }
            Some(Token::T_FUNCTION) => {
                match parse_function_statement(&mut token_iterator) {
                    Ok(func_stmt) => roots.push(Root::Func(func_stmt)),
                    Err(e) => {
                        panic!("Error parsing function statement: {:?}", e);
                    }
                }
            }
            Some(other) => {
                panic!("Unexpected token: {:?}", other);
            }
            None => break,
        }
    }

    return roots;
}