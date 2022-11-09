/*
scan an string input?
parse input
return list of tokens

https://mohitkarekar.com/posts/pl/lexer/

look at scanner / lexer from code
 */

use crate::lexer::lexer::LexerState::Start;
use crate::lexer::token::{find_kind, is_space, Kind};
use crate::lexer::token::Token;

// this may not be needed
enum LexerState {
    Start,
    End,
}

pub struct Lexer {
    state: LexerState,
    pub tokens: Vec<Token>,
    buffer: Vec<char>,
    index: usize
}

impl Lexer {
    pub fn new(buff: Vec<char>) -> Lexer {
        Lexer {
            state: Start,
            tokens: Vec::new(),
            buffer: buff,
            index: 0
        }
    }

    fn peek(&self) -> Result<char, &str> {
        return if self.index < self.buffer.len() {
            Ok(self.buffer[self.index])
        } else {
            Err("indexing error")
        }
    }

    fn get(&mut self) -> Result<char, &str> {
        return if self.index < self.buffer.len() {
            let ret = self.buffer[self.index];
            self.inc();
            Ok(ret)
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

    pub fn parse(&mut self) {
        let mut t_buf: String = "".to_owned();
        while let check = self.get() {
            match check {
                Ok(c) => {
                    if is_space(&c.to_string()) {
                        self.handle_buffer(&t_buf);
                        t_buf = "".to_owned();
                        continue;
                    }
                    t_buf.push(c);
                    ()
                }
                Err(msg) => {
                    self.handle_buffer(&t_buf);
                    break;
                }
            }
        }
    }

    fn handle_buffer(&mut self, buffer: &str) {
        let t_kind = find_kind(buffer);
        let t_token = Token::new(t_kind, buffer.clone(), self.index - buffer.len() - 1);
        self.tokens.push(t_token);
    }

    /*
        1. take in char stream
        2. read until end of stream
            2a. read in char and add to buffer.
            2b. once a space is encountered, stop adding to buffer then evaluate.
     */
}