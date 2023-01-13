use crate::lexer::token::Token;

pub struct Parser {
    input: Vec<Token>
}

impl Parser {
    pub fn new(lexemes: Vec<Token>) -> Parser {
        Parser {
            input: lexemes
        }
    }

    pub fn parse() {
        
    }
}