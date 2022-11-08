/*
scan an string input?
parse input
return list of tokens

https://mohitkarekar.com/posts/pl/lexer/

look at scanner / lexer from code
 */

use regex::Regex;
use crate::lexer::lexer::LexerState::Start;
use crate::lexer::token::Kind;
use crate::lexer::token::Token;

// this may not be needed
enum LexerState {
    Start,
    End,
}

struct Lexer {
    state: LexerState,

    // buffers.
    identifier: Token,
    number: Token,
    slash_or_comment: Token,
    atom: Token,

    buffer: Vec<char>,
    index: usize
}

impl Lexer {
    fn new(buff: Vec<char>) -> Lexer {
        Lexer {
            state: Start,
            identifier: Token::new(Kind::Identifier, "", 0),
            number: Token::new(Kind::Number, "", 0),
            slash_or_comment: Token::new(Kind::Slash, "", 0),
            atom: Token::new(Kind::Atom, "", 0),
            buffer: buff,
            index: 0
        }
    }

    fn peek(&self) -> char {
        return self.buffer[self.index];
    }

    fn get(&mut self) -> char {
        let ret: char = self.buffer[self.index];
        self.index = self.index + 1;
        return ret;
    }
}

fn is_space(c: char) -> bool {
    let re = Regex::new(r"").unwrap();
    return re.is_match("");
}