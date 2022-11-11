/*
scan an string input?
parse input
return list of tokens

https://mohitkarekar.com/posts/pl/lexer/

look at scanner / lexer from code
 */

use std::borrow::Borrow;
use regex::Regex;
use crate::lexer::lexer::LexerState::Start;
use crate::lexer::token::{is_space, build_simple_dictionary, build_complex_dictionary, find_kind, Kind};
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
    index: usize,
    line_number: usize,
    line_position: usize,
    complex_dict: Vec<(Regex, Kind)>,
    simple_dict: Vec<(String, Kind)>
}

impl Lexer {
    pub fn new(buff: Vec<char>) -> Lexer {
        Lexer {
            state: Start,
            tokens: Vec::new(),
            buffer: buff,
            index: 0,
            line_number: 0,
            line_position: 0,
            complex_dict: build_complex_dictionary(),
            simple_dict: build_simple_dictionary()
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

    pub fn parse(&mut self) {
        let mut t_buf: String = "".to_owned();
        while let check = self.get() {
            match check {
                Ok(c) => {
                    if is_space(&c.to_string()) {
                        if !t_buf.is_empty() {
                            self.handle_buffer(&t_buf);
                        }

                        t_buf = "".to_owned();
                    } else {
                        t_buf.push(c);
                    }

                    self.line_position += 1;
                    if c == '\n' {
                        self.line_number += 1;
                        self.line_position = 0;
                    }
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
        if let Some(t_kind) = find_kind(self.complex_dict.clone(), self.simple_dict.clone(), buffer) {
            let t_token = Token::new(
                t_kind,
                buffer.clone(),
                self.line_number,
                self.line_position - buffer.len());
            self.tokens.push(t_token);
        } else {
            println!("was not able to derive kind from buffer: {}", buffer);
        }
    }

    /*
        1. take in char stream
        2. read until end of stream
            2a. read in char and add to buffer.
            2b. once a space is encountered, stop adding to buffer then evaluate.
     */
}