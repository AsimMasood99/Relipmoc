use crate::lexer::tokens::Token;

#[derive(Debug)]
pub enum Errors {
    UnexpectedEOF,
    FailedToFindToken(Token),
    ExpectedTypeToken(Token),
    ExpectedIdentifier(Token),
    UnexpectedToken(Token),
    ExpectedFloatLit,
    ExpectedIntLit,
    ExpectedStringLit,
    ExpectedBoolLit,
    ExpectedExpr,
    InvalidAssignmentTarget,
}
