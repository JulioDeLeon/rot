/*
scan an string input?
parse input
return list of tokens

https://mohitkarekar.com/posts/pl/lexer/

look at scanner / lexer from code
 */

use crate::lexer::lexer::LexerState::Start;
use crate::lexer::token::{is_space, Kind};
use crate::lexer::token::Token;

// this may not be needed
enum LexerState {
    Start,
    End,
}

struct Lexer {
    state: LexerState,
    tokens: Vec<Token>,
    buffer: Vec<char>,
    index: usize
}

impl Lexer {
    fn new(buff: Vec<char>) -> Lexer {
        Lexer {
            state: Start,
            tokens: Vec::new(),
            buffer: buff,
            index: 0
        }
    }

    fn peek(&self) -> Result<char, &str> {
        return if self.index < self.buffer.capacity() {
            Ok(self.buffer[self.index])
        } else {
            Err("indexing error")
        }
    }

    fn get(&mut self) -> Result<char, &str> {
        return if self.index < self.buffer.capacity() {
            self.inc();
            Ok(self.buffer[self.index])
        } else {
            Err("indexing error")
        };
    }

    fn inc(&mut self) {
        self.index += 1;
    }

    fn dec(&mut self) {
        self.index -= 1;
    }

    fn parse(&mut self) {
        let mut t_buf: String = "".to_owned();

        while let Ok(c) = self.get() {
            if is_space(c) {
                self.handle_buffer(&t_buf);
                continue;
            }
        }
    }

    fn handle_buffer(&mut self, buffer: &str) {
        // temp into basic token
        let t_token = Token::new(Kind::Atom, buffer, self.index - buffer.len());
        self.tokens.push(t_token);
    }

    /*
        1. take in char stream
        2. read until end of stream
            2a. read in char and add to buffer.
            2b. once a space is encountered, stop adding to buffer then evaluate.
     */
}