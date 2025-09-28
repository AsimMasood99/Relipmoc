use crate::parser::enums::{*};
use crate::lexer::tokens::Token;
use crate::parser::token_iterator::TokenIterator;
use crate::parser::errors::Errors;

fn parse_expression(tokens: &mut TokenIterator) -> Result<Expression, Errors> {
    parse_logical_or(tokens)
}

fn parse_logical_or(tokens: &mut TokenIterator) -> Result<Expression, Errors> {
    let mut expr = parse_logical_and(tokens)?;
    
    while let Some(token) = tokens.peek_curr() {
        match token {
            Token::T_OR_OPR => {
                tokens.consume()?;
                let right = parse_logical_and(tokens)?;
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    operator: Token::T_OR_OPR,
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }
    
    Ok(expr)
}

fn parse_logical_and(tokens: &mut TokenIterator) -> Result<Expression, Errors> {
    let mut expr = parse_equality(tokens)?;
    
    while let Some(token) = tokens.peek_curr() {
        match token {
            Token::T_AND_OPR => {
                tokens.consume()?;
                let right = parse_equality(tokens)?;
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    operator: Token::T_AND_OPR,
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }
    
    Ok(expr)
}

fn parse_equality(tokens: &mut TokenIterator) -> Result<Expression, Errors> {
    let mut expr = parse_comparison(tokens)?;
    
    while let Some(token) = tokens.peek_curr() {
        match token {
            Token::T_EQUALS_OPR => {
                tokens.consume()?;
                let right = parse_comparison(tokens)?;
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    operator: Token::T_EQUALS_OPR,
                    right: Box::new(right),
                };
            }
            Token::T_NOT_EQUALS_OPR => {
                tokens.consume()?;
                let right = parse_comparison(tokens)?;
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    operator: Token::T_NOT_EQUALS_OPR,
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }
    
    Ok(expr)
}

fn parse_comparison(tokens: &mut TokenIterator) -> Result<Expression, Errors> {
    let mut expr = parse_shift(tokens)?;
    
    while let Some(token) = tokens.peek_curr() {
        match token {
            Token::T_GREATER_THAN_OPR => {
                tokens.consume()?;
                let right = parse_shift(tokens)?;
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    operator: Token::T_GREATER_THAN_OPR,
                    right: Box::new(right),
                };
            }
            Token::T_LESS_THAN_OPR => {
                tokens.consume()?;
                let right = parse_shift(tokens)?;
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    operator: Token::T_LESS_THAN_OPR,
                    right: Box::new(right),
                };
            }
            Token::T_GREATER_THAN_EQUAL_TO_OPR => {
                tokens.consume()?;
                let right = parse_shift(tokens)?;
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    operator: Token::T_GREATER_THAN_EQUAL_TO_OPR,
                    right: Box::new(right),
                };
            }
            Token::T_LESS_THAN_EQUAL_TO_OPR => {
                tokens.consume()?;
                let right = parse_shift(tokens)?;
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    operator: Token::T_LESS_THAN_EQUAL_TO_OPR,
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }
    
    Ok(expr)
}

fn parse_shift(tokens: &mut TokenIterator) -> Result<Expression, Errors> {
    let mut expr = parse_term(tokens)?;
    
    while let Some(token) = tokens.peek_curr() {
        match token {
            Token::T_LEFT_SHIFT_OPR => {
                tokens.consume()?;
                let right = parse_term(tokens)?;
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    operator: Token::T_LEFT_SHIFT_OPR,
                    right: Box::new(right),
                };
            }
            Token::T_RIGHT_SHIFT_OPR => {
                tokens.consume()?;
                let right = parse_term(tokens)?;
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    operator: Token::T_RIGHT_SHIFT_OPR,
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }
    
    Ok(expr)
}

fn parse_term(tokens: &mut TokenIterator) -> Result<Expression, Errors> {
    let mut expr = parse_factor(tokens)?;
    
    while let Some(token) = tokens.peek_curr() {
        match token {
            Token::T_PLUS_OPR => {
                tokens.consume()?;
                let right = parse_factor(tokens)?;
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    operator: Token::T_PLUS_OPR,
                    right: Box::new(right),
                };
            }
            Token::T_MINUS_OPR => {
                tokens.consume()?;
                let right = parse_factor(tokens)?;
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    operator: Token::T_MINUS_OPR,
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }
    
    Ok(expr)
}

fn parse_factor(tokens: &mut TokenIterator) -> Result<Expression, Errors> {
    let mut expr = parse_exponential(tokens)?;
    
    while let Some(token) = tokens.peek_curr() {
        match token {
            Token::T_MULTIPLY_OPR => {
                tokens.consume()?;
                let right = parse_exponential(tokens)?;
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    operator: Token::T_MULTIPLY_OPR,
                    right: Box::new(right),
                };
            }
            Token::T_DIVIDE_OPR => {
                tokens.consume()?;
                let right = parse_exponential(tokens)?;
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    operator: Token::T_DIVIDE_OPR,
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }
    
    Ok(expr)
}

fn parse_exponential(tokens: &mut TokenIterator) -> Result<Expression, Errors> {
    let mut expr = parse_unary(tokens)?;
    
    while let Some(token) = tokens.peek_curr() {
        match token {
            Token::T_EXPONENT_OPR => {
                tokens.consume()?;
                let right = parse_unary(tokens)?;
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    operator: Token::T_EXPONENT_OPR,
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }
    
    Ok(expr)
}

fn parse_unary(tokens: &mut TokenIterator) -> Result<Expression, Errors> {
    if let Some(token) = tokens.peek_curr() {
        match token {
            Token::T_NOT | Token::T_MINUS_OPR => {
                let operator = tokens.consume()?.clone();
                let expression = parse_unary(tokens)?;
                return Ok(Expression::UnaryOperation {
                    operator: operator,
                    expression: Box::new(expression),
                });
            }
            _ => {}
        }
    }
    
    parse_primary(tokens)
}

fn parse_primary(tokens: &mut TokenIterator) -> Result<Expression, Errors> {
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
        Token::T_IDENTIFIER(name) => {
            Ok(Expression::Identifier(name.clone()))
        }
        Token::T_ROUND_BRACKET_OPEN => {
            let expr = parse_expression(tokens)?;
            tokens.seek_if(Token::T_ROUND_BRACKET_CLOSE)?;
            Ok(expr)
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