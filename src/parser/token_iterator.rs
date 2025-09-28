use crate::lexer::tokens::Token;
use crate::parser::errors::Errors;

#[derive(Debug)]
pub struct TokenIterator {
    position: usize,
    stream: Vec<Token>,
}

impl TokenIterator {
    pub fn peek_curr(&self) -> Option<&Token> {
        self.stream.get(self.position)
    }

    pub fn peek_next(&self) -> Option<&Token> {
        self.stream.get(self.position + 1)
    }

    pub fn seek_if(&mut self, expected: Token) -> Result<(), Errors> {
        match self.peek_curr() {
            Some(token) => {
                if *token == expected {
                    self.position += 1;
                    Ok(())
                } else {
                    Err(Errors::UnexpectedToken(token.clone()))
                }
            }
            None => Err(Errors::UnexpectedEOF),
        }
    }

    pub fn consume(&mut self) -> Result<&Token, Errors> {
        let token = self.stream.get(self.position).ok_or(Errors::UnexpectedEOF)?;
        self.position += 1;
        Ok(token)
    }

    pub fn is_at_end(&self) -> bool {
        self.position >= self.stream.len()
    }

    pub fn new(stream: Vec<Token>) -> Self {
        TokenIterator { position: 0, stream }
    }
}