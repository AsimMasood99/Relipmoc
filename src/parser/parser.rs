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
        Token::T_STRINGLIT(value) => {
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
            Some(other) => {
                panic!("Unexpected token: {:?}", other);
            }
            None => break,
        }
    }

    return roots;
}