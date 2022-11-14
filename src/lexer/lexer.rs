/*
scan an string input?
parse input
return list of tokens

https://mohitkarekar.com/posts/pl/lexer/

look at scanner / lexer from code
 */

use core::fmt;
use std::borrow::Borrow;
use regex::Regex;
use crate::lexer::lexer::LexerState::Start;
use crate::lexer::token::{is_space, build_simple_dictionary, build_complex_dictionary, find_kind, Kind, SimpleDict, ComplexDict};
use crate::lexer::token::Token;

#[derive(Eq, Clone)]
enum LexerState {
    Start,
    StringEval,
    CommentEval,
    MultiLnStringEval,
    MultiLnCommentEval,
    CharEval,
    RegexEval,
    KeywordEval,
    Error(String),
    End,
}

impl fmt::Display for LexerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let LexerState::Error(msg) = self {
            return write!(f, "{}", format!("LexerState::Error({})", msg))
        }

        let val = match self {
            Start => "Start",
            StringEval => "StringEval",
            MultiLnStringEval => "MultiLnStringEval",
            CommentEval => "CommentEval",
            MultiLnCommentEval => "MultiLnCommentEval",
            CharEval => "CharEval",
            RegexEval => "RegexEval",
            KeywordEval => "KeywordEval",
            End => "End"
        };
        write!(f, "{}", val)
    }
}

pub struct Lexer {
    state: LexerState,
    pub tokens: Vec<Token>,
    buffer: Vec<char>,
    index: usize,
    line_number: usize,
    line_position: usize,
    complex_dict: ComplexDict,
    simple_dict: SimpleDict,
    workspace: String
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
            simple_dict: build_simple_dictionary(),
            workspace: "".to_owned(),
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
        while self.state != LexerState::End {
            let new_state = self.handleState(&t_buf);
            if let LexerState::Error(msg) = new_state {
                println!("Encountered an error while parsing: {}", LexerState::Error(msg));
                break;
            }
            self.state = new_state
        }
    }

    pub fn reset(&mut self, buff: Vec<char>) {
        self.state = Start;
        self.buffer = buff;
        self.index = 0;
        self.line_position = 0;
        self.line_number = 0;
        // clear tokens?
    }

    fn handleState(&mut self, buffer: &str) -> LexerState {
        let snapshot_state: LexerState = self.state.clone();
        return match snapshot_state {
            Start => self.handle_start_state(buffer),
            KeywordEval => self.handle_keyword_eval(buffer),
            CommentEval => self.handle_comment_eval(buffer),
            StringEval => self.handle_string_eval(buffer),
            MultiLnStringEval => self.handle_multilnstring_eval(buffer),
            MultiLnCommentEval => self.handle_multilncomment_eval(buffer),
            CharEval => self.handle_char_eval(buffer),
            RegexEval=> self.handle_regex_eval(buffer),
            End => End,
            LexerState::Error(msg) => {
                println!("Lexer in error state: {}", LexerState::Error(msg));
                LexerState::End
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

    fn handle_start_state(&mut self, buffer: &str) -> LexerState {
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
        LexerState::End
    }

    fn handle_comment_eval(&mut self, buffer: &str) -> LexerState {
        LexerState::End
    }

    fn handle_string_eval(&mut self, buffer: &str) -> LexerState {
        LexerState::End
    }

    fn handle_multilnstring_eval(&mut self, buffer: &str) -> LexerState {
        LexerState::End
    }

    fn handle_multilncomment_eval(&mut self, buffer: &str) -> LexerState {
        LexerState::End
    }

    fn handle_keyword_eval(&mut self, buffer: &str) -> LexerState {
        LexerState::End
    }

    fn handle_regex_eval(&mut self, buffer: &str) -> LexerState {
        LexerState::End
    }

    fn handle_char_eval(&mut self, buffer: &str) -> LexerState {
        LexerState::End
    }
}