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

    // now we will go into the block

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








