include! ("../AST/enums.rs")

#[derive(Debug)]
pub enum Errors{
    UnexpectedEOF,
    FailedToFindToken(Token),
    ExpectedTypeToken,
    ExpectedIdentifier,
    UnexpectedToken(Token),
    ExpectedFloatLit,
    ExpectedIntLit,
    ExpectedStringLit,
    ExpectedBoolLit,
    ExpectedExpr,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    String,
    Float,
    Bool,
}



fn params_parser<'a>(parser: &mut Parser<'a>) -> Result<FunctionDeclaration, Errors>{
    let mut params: Vec<param> = Vec::new();

    // we have read the opening paranthese 
    parser.position+=1;
    // now checking if the next one is a closing paranthese
    if parser.peek_next == Token::T_ROUND_BRACKET_CLOSE
    {
        return Ok(params) // empty vector is returned
    }

    // -------------------------------------
    loop{
        // parse param (identifire and token_type[int, string, etc])
        // 1st parse the parameter type
        let param_type = match parser.consume()? {
            Token::T_INT => Type::Int,
            Token::T_STRING => Type::String,
            Token::T_FLOAT => Type::Float,
            Token::T_BOOL => Type::Bool,
            other => return Err(Errors::ExpectedTypeToken(other.clone())),
        }
        
        
        // 2nd checking the identifier 
        let param_identifier = match parser.consume()?{
            Token::T_IDENTIFIER(name) => name.clone(), // matching and if it matched copy (clone) the name
            other => return Err(Errors::ExpectedIdentifier(other.clone())) // no match: send error
        }
        
        // TODO: will we be including an equal sign?

        // 3rd now we are adding this parameter to the
        params.push(param {
            type_token: param_type,
            identifier: param_identifier,
        });

        // 5th now checking for next semi-colon or end bracket
        match param.consume()?
        {
            Some(Token::T_ROUND_BRACKET_CLOSE)=>{
                break; // all parameters dealth with
            }
            Some(Token::T_COMMA)=>
            {
                continue;
            }
        }
    }
    Ok(params)
}

// block parser

fn block_parser<'a>(parser: &mut Parser<'a>) -> Result<FunctionDeclaration, Errors>{

    loop{
        
    }

}

fn function_declaration<'a>(parser: &mut Parser<'a>, &mut ) -> Result<FunctionDeclaration, Errors> {
    
    let new_Function = function_statement{}

    // first token should be T_FUNCTION which we already checked before calling this function
    parser.position += 1; // move to next token

    // 1st will be the function return type
    let return_type = match parser.consume()?{
        Token::T_INT => Type::Int,
            Token::T_STRING => Type::String,
            Token::T_FLOAT => Type::Float,
            Token::T_BOOL => Type::Bool,
            // TODO: Add void return type here 
            other => return Err(Errors::ExpectedTypeToken(other.clone())),
    }

    // 2nd will be the function identifier (my_function, etc)
    let identifier = match parser.consume()? {
        Token::T_IDENTIFIER(name) => name.clone(),
        other => return Err(Errors::ExpectedIdentifier(other.clone())),
    };

    // 3rd will be the check for round bracket open (else give error)
    parser.peek_next_with_caution(Token:T_ROUND_BRACKET_OPEN)
    
    // 4th will be the parameters fow which we have a dedicated function
    let all_parames = params_parser(&mut parser)?;

    // 5th now we will see a closed round braces
    parser.peek_next_with_caution(Token:T_ROUND_BRACKET_CLOSE)
    
    // now curly braces open
    parser.peek_next_with_caution(Token:T_CURLY_BRACKET_OPEN)

    // TODO: now we will go into the block
    
}


// variable decleration parser
fn variable_declaration_parser<'a>(tokens: &'a Vec<Token>) -> Result<Vec<root>, Errors>
{   let var = variable_declaration{};
    
    let var_type = match parser.consume()? {
            Token::T_INT => Type::Int,
            Token::T_STRING => Type::String,
            Token::T_FLOAT => Type::Float,
            Token::T_BOOL => Type::Bool,
            other => return Err(Errors::ExpectedTypeToken(other.clone())),
        }
        
    // 2nd checking the identifier 
    let var_identifier = match parser.consume()?{
        Token::T_IDENTIFIER(name) => name.clone(), // matching and if it matched copy (clone) the name
        other => return Err(Errors::ExpectedIdentifier(other.clone())) // no match: send error
    }
    
    // 3rd is equal sign
    // params.peek_next_with_caution(Token::T_EQUALS_OPR)
    
    // 3rd is the equal sign
    // 4th is the expression (number that is being assigned)
    // +++++  (The expression will handle the equal sign and the number) ++++++
    // TODO: add expression-statement here (for the number)

    // 3rd now we are adding this parameter to the
    ok (var{
        type_token: var_type,
        identifier: var_identifier,
        expression: // TODO: returned expression will be added here
    })
}

// for statement parser
fn for_loop_parser<'a>(tokens: &'a Vec<Token>) -> Result<Vec<root>, Errors> {
    // for loop is detected (for token has been read)
    parser.position += 1; // move to next token


    // 1st we will be expecting an open round braces
    parser.peek_next_with_caution(Token::T_ROUND_BRACKET_OPEN)

    // 2nd we are expeting the loop variable decleration
    let init_loop_variable = if parser.peek_curr() == Some(Token::T_SEMICOLON)
    {
        // this means there is no variable to initialize
        None;
    }
    else{   
        let returned_loop_var = variable_declaration_parser(&mut parser)?;
        Some(returned_loop_var)
    }

    // 3rd we are expecting a semi-colon
    parser.peek_next_with_caution(Token::T_SEMICOLON)

    // 4th we are expecting the loop condition (expression statement)
    let loop_condition = expression_statement_parser(&mut parser)?; // TODO: write this function

    // 5th we are expecting a semi-colon
    parser.peek_next_with_caution(Token::T_SEMICOLON)

    // 6th we are expecting the update loop variable (expression)
    let update_loop_var = if parser.peek_curr() == Some(Token::T_ROUND_BRACKET_CLOSE)
    {
        // this means there is no variable to initialize
        None
    }
    else{   
        let returned_expr = expression_parser(&mut parser)?; // TODO: write this function
        Some(returned_expr) 
    }

    // 7th we are expecting a closing round braces
    parser.peek_next_with_caution(Token::T_ROUND_BRACKET_CLOSE) 

    // 8th we are expecting a block (curly braces and statements inside it)
    let block = block_parser(&mut parser)?; // TODO: write this function

    Ok(Root::ForLoop {
        init_variable: init_loop_variable,
        condition: loop_condition,
        update: update_loop_var,
        body: block,
    })
}


// helper of if statement parser
fn if_statement_expression<'a>(tokens: &'a Vec<Token>) -> Result<Vec<root>, Errors> {

    let if_expr = if_statement_expression{};

    // first token should be T_IF which we already checked before calling this function
    // parser.position += 1; // move to next token

    // 1st we are expecting an open round braces
    parser.peek_next_with_caution(Token::T_ROUND_BRACKET_OPEN)

    // 2nd we are expecting the condition expression
    let condition = expression_statement_parser(&mut parser)?; // TODO: write this function

    // 3rd we are expecting a closing round braces
    parser.peek_next_with_caution(Token::T_ROUND_BRACKET_CLOSE)

    // 4th we are expecting a block (curly braces and statements inside it)
    let block = block_parser(&mut parser)?; // TODO: write this function

    Ok(if_statement_expression{
        condition,
        block,
    })
}


// helper of if statement parser
fn elif_statement<'a>(tokens: &'a Vec<Token>) -> Result<Vec<root>, Errors> {
    let mut elif_statements: Vec<elif_statement> = Vec::new();
    //++++++++++++++++++ TODO: complete this function (its confusing) ++++++++++++++


// if statement parser
fn if_statement_parser<'a>(tokens: &'a Vec<Token>) -> Result<Vec<root>, Errors> {
    // first token should be T_IF which we already checked before calling this function
    parser.position += 1; // move to next token


    let if_expr = if_statement_expression(&mut parser)?;

    // 2nd is the elif statements (zero or more)
    let mut elif_statements = elif_statement(&mut parser)?;

    // 3rd is the else statement (optional)
    let else_statement = if parser.peek_curr() == Some(Token::T_ELSE)
    {
        parser.position += 1; // move to next token
        Some(block_parser(&mut parser)?) // parsing the block after else
    }
    else{
        None
    }

    Ok(Root::IfStatement {
        if_expression: if_expr,
        elif_expressions: elif_statements,
        else_expression: else_statement,
    })
}

fn return_statement_parser<'a>(tokens: &'a Vec<Token>) -> Result<Vec<root>, Errors> {
    // we have read the return token already and moving to next token
    parser.position += 1;

    // now we are expecting an expression statement
    let expr = expression_statement_parser(&mut parser)?; // TODO: write this function

    Ok(Root::ReturnStatement {
        expr,
    })
}

fn break_statement_parser<'a>(tokens: &'a Vec<Token>) -> Result<Vec<root>, Errors> {
    // we have read the break token already and moving to next token
    parser.position += 1;

    Ok(Root::BreakStatement)

    // no need to check for semi-colon as it is not required after break
}

// entry point of parser
fn parser<'a>(tokens: &'a Vec<Token>) -> Result<Vec<root>, Errors> {
    
    let mut parser = Parser {
        position: 0,
        stream: tokens,
    };

    fn peek_curr(&self) -> Option<&Token> {
        self.stream.get(self.position)
    }

    fn peek_next(&self) -> Option<&Token> {
        self.stream.get(self.position + 1)
    }

    // Expect a specific token or return an error
    fn  peek_next_with_caution(&mut self, expected: Token) -> Result<(), Errors> {
        match self.peek() {
            Some(token) if *token == expected => {
                self.consume()?;
                Ok(())
            }
            Some(other) => Err(Errors::UnexpectedToken(other.clone())),
            None => Err(Errors::UnexpectedEOF),
        }
    }

    // Consume the current token by advancing and returning a reference to it the current for use
    fn consume(&mut self) -> Result<&Token, Errors> {
        let token = self.stream.get(self.position).ok_or(Errors::UnexpectedEOF)?;
        self.position += 1;
        Ok(token)
    }


    // parsing begines here
    let mut roots: Vec<root> = Vec::new(); // this will be our AST vector
    while parser.position < parser.stream.len() { // going through all tokens

        match current_token.token_type.as_str() { // Assuming token_type is a String or &str
            Some("T_FUNCTION") => {
                // Parse the function declaration.
                let function_node = function_declaration(&mut parser)?;
                // Add the resulting AST node to our tree.
                ast.push(RootNode::Function(function_node)); 
            }
            _ => {
                // Syntax error: We found a token we don't know how to handle at the top level.
                return Err(Errors::UnexpectedToken(current_token.clone()));
            }

        }
    }
}








