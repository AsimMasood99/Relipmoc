use crate::parser::enums::{*};
use crate::lexer::tokens::Token;
use crate::parser::token_iterator::TokenIterator;
use crate::parser::errors::Errors;

fn parse_expression(tokens: &mut TokenIterator) -> Result<Expression, Errors> {
    parse_assignment(tokens)
}

fn parse_assignment(tokens: &mut TokenIterator) -> Result<Expression, Errors> {
    let expr = parse_logical_or(tokens)?;
    
    if let Some(Token::T_ASSIGNMENT_OPR) = tokens.peek_curr() {
        tokens.consume()?;
        let right = parse_assignment(tokens)?;
        
        // Ensure left side is an identifier for assignment
        if let Expression::Identifier(_) = expr {
            return Ok(Expression::Assignment {
                left: Box::new(expr),
                right: Box::new(right),
            });
        } else {
            return Err(Errors::InvalidAssignmentTarget);
        }
    }
    
    Ok(expr)
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
    // Check for identifier first to handle function calls properly
    if let Some(Token::T_IDENTIFIER(_)) = tokens.peek_curr() {
        if let Some(Token::T_ROUND_BRACKET_OPEN) = tokens.peek_next() {
            // Function call - don't consume the identifier here, let parse_function_call handle it
            let func_call = parse_function_call(tokens)?;
            return Ok(Expression::FunctionCall(func_call));
        } else {
            // Regular identifier
            let name = match tokens.consume()? {
                Token::T_IDENTIFIER(name) => name.clone(),
                other => return Err(Errors::ExpectedIdentifier(other.clone())),
            };
            return Ok(Expression::Identifier(name));
        }
    }

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
    tokens.seek_if(Token::T_FUNCTION)?;

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

    // { block start
    tokens.seek_if(Token::T_CURLY_BRACKET_OPEN)?;

    let block = parse_block(tokens)?;

    // } block end
    tokens.seek_if(Token::T_CURLY_BRACKET_CLOSE)?;

    Ok(FunctionStatement {
        return_type,
        identifier: func_name,
        parameters,
        block,
    })
}

fn parse_block(tokens: &mut TokenIterator) -> Result<Block, Errors> {
    let mut statements = Vec::new();

    while let Some(token) = tokens.peek_curr() {
        match token {
            Token::T_INT | Token::T_FLOAT | Token::T_BOOL | Token::T_STRING => {
                let var_decl = parse_variable_declaration(tokens)?;
                statements.push(Statement::VarDecl(var_decl));
            }
            Token::T_IF => {
                let if_stmt = parse_if_statement(tokens)?;
                statements.push(Statement::If(if_stmt));
            }
            Token::T_FOR => {
                let for_stmt = parse_for_statement(tokens)?;
                statements.push(Statement::For(for_stmt));
            }
            Token::T_WHILE => {
                let while_stmt = while_loop_parser(tokens)?;
                statements.push(Statement::While(while_stmt));
            }
            Token::T_RETURN => {
                let ret_stmt = parse_return_statement(tokens)?;
                statements.push(Statement::Return(ret_stmt));
            }
            Token::T_CURLY_BRACKET_CLOSE => break, // End of block
            _ => {
                // Try parsing as an expression statement (e.g., function call)
                let expr = parse_expression(tokens)?;
                statements.push(Statement::Expr(expr));
            }
        }
    }
    
    Ok(Block { statements })
}

fn parse_if_statement(tokens: &mut TokenIterator) -> Result<IfStatement, Errors> {
    tokens.seek_if(Token::T_IF)?;
    
    // opening bracket
    tokens.seek_if(Token::T_ROUND_BRACKET_OPEN)?;
    
    // condition expression
    let condition = parse_expression(tokens)?;
    
    // closing bracket
    tokens.seek_if(Token::T_ROUND_BRACKET_CLOSE)?;
    
    // { block start
    tokens.seek_if(Token::T_CURLY_BRACKET_OPEN)?;
    
    let block = parse_block(tokens)?;
    
    // } block end
    tokens.seek_if(Token::T_CURLY_BRACKET_CLOSE)?;
    
    // Parse optional elif blocks
    let elif_blocks = parse_elif_blocks(tokens)?;
    
    // Parse optional else block
    let else_block = parse_else_block(tokens)?;
    
    Ok(IfStatement {
        condition,
        block,
        elif_blocks,
        else_block,
    })
}


fn while_loop_parser(tokens: &mut TokenIterator) -> Result<WhileStatement, Errors> {
    tokens.seek_if(Token::T_WHILE)?;

    // opening bracket
    tokens.seek_if(Token::T_ROUND_BRACKET_OPEN)?;

    // condition expression
    let condition = parse_expression(tokens)?;

    // closing bracket
    tokens.seek_if(Token::T_ROUND_BRACKET_CLOSE)?;

    // { block start
    tokens.seek_if(Token::T_CURLY_BRACKET_OPEN)?;

    let block = parse_block(tokens)?;

    // } block end
    tokens.seek_if(Token::T_CURLY_BRACKET_CLOSE)?;

    Ok(WhileStatement {
        condition,
        block,
    })

}

fn parse_elif_blocks(tokens: &mut TokenIterator) -> Result<Vec<ElifBlock>, Errors> {
    let mut elif_blocks = Vec::new();
    
    while let Some(Token::T_ELSE_IF) = tokens.peek_curr() {
        tokens.consume()?; // consume ELIF
        
        // opening bracket
        tokens.seek_if(Token::T_ROUND_BRACKET_OPEN)?;
        
        // condition expression
        let elif_condition = parse_expression(tokens)?;

        // closing bracket
        tokens.seek_if(Token::T_ROUND_BRACKET_CLOSE)?;
        
        // { block start
        tokens.seek_if(Token::T_CURLY_BRACKET_OPEN)?;
        
        let elif_block = parse_block(tokens)?;
        
        // } block end
        tokens.seek_if(Token::T_CURLY_BRACKET_CLOSE)?;
        
        elif_blocks.push(ElifBlock {
            condition: elif_condition,
            block: elif_block,
        });
    }
    
    Ok(elif_blocks)
}

fn parse_else_block(tokens: &mut TokenIterator) -> Result<Option<Block>, Errors> {
    if let Some(Token::T_ELSE) = tokens.peek_curr() {
        tokens.consume()?; // consume ELSE
        
        // { block start
        tokens.seek_if(Token::T_CURLY_BRACKET_OPEN)?;
        
        let block = parse_block(tokens)?;
        
        // } block end
        tokens.seek_if(Token::T_CURLY_BRACKET_CLOSE)?;
        
        Ok(Some(block))
    } else {
        Ok(None)
    }
}

fn parse_for_statement(tokens: &mut TokenIterator) -> Result<ForStatement, Errors> {
    tokens.consume()?; // consume 'for'

    // consume '('
    tokens.seek_if(Token::T_ROUND_BRACKET_OPEN)?;
    
    // Parse init_loop_var (variable declaration or empty semicolon)
    let init_var = match tokens.peek_curr() {
        Some(Token::T_SEMICOLON) => {
            tokens.consume()?; // consume semicolon
            None // empty initialization
        } // Otherwise parse variable declaration. 
        Some(Token::T_INT) | Some(Token::T_FLOAT) | Some(Token::T_BOOL) | Some(Token::T_STRING) => {
            let var_decl = parse_variable_declaration(tokens)?;
            Some(var_decl)
        }
        other => return Err(Errors::UnexpectedToken(
            other.cloned().unwrap_or(Token::T_SEMICOLON)
        )),
    };
    
    // Parse loop_condition: 
    let condition = if let Some(Token::T_SEMICOLON) = tokens.peek_curr() {
        tokens.consume()?; // consume semicolon
        None // empty condition
    } else { // Else parse condition expression 
        let expr = parse_expression(tokens)?;
        tokens.seek_if(Token::T_SEMICOLON)?;
        Some(expr)
    };
    
    let update = if let Some(Token::T_ROUND_BRACKET_CLOSE) = tokens.peek_curr() {
        None //no update. 
    } else {
        Some(parse_expression(tokens)?)
    };
    
    tokens.seek_if(Token::T_ROUND_BRACKET_CLOSE)?;
    
    tokens.seek_if(Token::T_CURLY_BRACKET_OPEN)?;
    
    let block = parse_block(tokens)?;
    
    // Parse block closing
    tokens.seek_if(Token::T_CURLY_BRACKET_CLOSE)?;
    
    Ok(ForStatement {
        init_var,
        condition,
        update,
        block,
    })
}

fn parse_return_statement(tokens: &mut TokenIterator) -> Result<Expression, Errors> {
    tokens.seek_if(Token::T_RETURN)?;
    let expr = parse_expression(tokens)?;
    tokens.seek_if(Token::T_SEMICOLON)?;
    Ok(expr)
}

pub fn parse_function_call_arguments(tokens: &mut TokenIterator) -> Result<Vec<Expression>, Errors> {
    let mut args = Vec::new();

    // if next token is ')', then no arguments
    if let Some(Token::T_ROUND_BRACKET_CLOSE) = tokens.peek_curr() {
        return Ok(args);
    }

    loop {
        let expr = parse_expression(tokens)?;
        args.push(expr);

        if let Some(Token::T_COMMA) = tokens.peek_curr() {
            tokens.consume()?; // consume the comma
            continue;
        } else {
            break; // no more arguments
        }
    }

    Ok(args)
}

pub fn parse_function_call(tokens: &mut TokenIterator) -> Result<FunctionCallStatement, Errors> {
    let func_name = match tokens.consume()? {
        Token::T_IDENTIFIER(name) => name.clone(),
        other => return Err(Errors::ExpectedIdentifier(other.clone())),
    };

    tokens.seek_if(Token::T_ROUND_BRACKET_OPEN)?;

    let args = parse_function_call_arguments(tokens)?;

    tokens.seek_if(Token::T_ROUND_BRACKET_CLOSE)?;
    tokens.seek_if(Token::T_SEMICOLON)?;

    Ok(FunctionCallStatement {
        identifier: func_name,
        args,
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