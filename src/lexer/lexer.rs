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
use crate::lexer::lexer::LexerState::*;
use crate::lexer::token::{is_space, build_simple_dictionary, build_complex_dictionary, find_kind, Kind, SimpleDict, ComplexDict};
use crate::lexer::token::Token;

#[derive(PartialEq, Clone)]
enum LexerState {
    Start,
    NumericEval,
    StringEval,
    CommentEval,
    MultiLnStringEval,
    MultiLnCommentEval,
    CharEval,
    MaybeRegexEval,
    RegexEval,
    KeywordEval,
    Error(String),
    End,
}

impl fmt::Display for LexerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Error(msg) = self {
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
            End => "End",
            _ => "unknown state"
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
    workspace: String,
    last: Option<char>,
    curr: Option<char>,
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
            last: None::<char>,
            curr: None::<char>,
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
            self.last = self.curr;
            self.curr = Some(ret);
            self.line_position += 1;

            match self.last {
                Some(c) => {
                    if c == '\n' {
                        self.line_number += 1;
                        self.line_position = 0;
                    }
                },
                None => ()
            }

            Ok(ret)
        } else {
            Err("indexing error")
        };
    }

    fn inc(&mut self) {
        self.index += 1;
    }

    pub fn parse(&mut self) {
        while self.state != End {
            let new_state = self.handleState();
            if let Error(msg) = new_state {
                println!("Encountered an error while parsing: {}", Error(msg));
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

    fn handleState(&mut self) -> LexerState {
        let snapshot_state: LexerState = self.state.clone();
        return match snapshot_state {
            Start => self.handle_start_state(),
            KeywordEval => self.handle_keyword_eval(),
            CommentEval => self.handle_comment_eval(),
            StringEval => self.handle_string_eval(),
            MultiLnStringEval => self.handle_multilnstring_eval(),
            MultiLnCommentEval => self.handle_multilncomment_eval(),
            CharEval => self.handle_char_eval(),
            MaybeRegexEval=> self.handle_maybe_regex(),
            RegexEval=> self.handle_regex_eval(),
            NumericEval => self.handle_numeric_eval(),
            End => End,
            Error(msg) => {
                println!("Lexer in error state: {}", Error(msg));
                End
            }
        }
    }

    fn handle_buffer(&mut self) {
        if let Some(t_kind) = find_kind(self.complex_dict.clone(), self.simple_dict.clone(), self.workspace.clone()) {
            let t_token = Token::new(
                t_kind,
                self.workspace.clone(),
                self.line_number,
                self.line_position - self.workspace.len());
            self.tokens.push(t_token);
        } else {
            println!("was not able to derive kind from buffer: {}", self.workspace);
        }
    }

    fn handle_start_state_complex_case(&mut self, x: char) -> LexerState {
        let mut ret = End;

        if is_space(&x.to_string()) {
            if !self.workspace.is_empty() {
                self.handle_buffer();
            }
            self.workspace = "".to_owned();

            ret = Start
        } else {
            self.workspace.push(x);
            ret = KeywordEval
        }

        return ret
    }

    fn handle_start_state_simple_case(&mut self, x: char) -> LexerState {
        match x {
            'r' => MaybeRegexEval,
            '"' => StringEval,
            '#' => CommentEval,
            '\'' => CharEval,
            _ => self.handle_start_state_complex_case(x.clone()),
        }
    }

    fn handle_start_state(&mut self) -> LexerState {
        let check = self.get();

        return match check {
            Ok(c) => {
                self.handle_start_state_simple_case(c.clone())
            },
            _ => End
        };
    }

    fn handle_comment_eval(&mut self) -> LexerState {
        LexerState::End
    }

    fn handle_string_eval(&mut self) -> LexerState {
        LexerState::End
    }

    fn handle_multilnstring_eval(&mut self) -> LexerState {
        LexerState::End
    }

    fn handle_multilncomment_eval(&mut self) -> LexerState {
        LexerState::End
    }

    fn handle_keyword_eval(&mut self) -> LexerState {
        LexerState::End
    }

    fn handle_maybe_regex(&mut self) -> LexerState {
        LexerState::End
    }

    fn handle_regex_eval(&mut self) -> LexerState {
        LexerState::End
    }

    fn handle_char_eval(&mut self) -> LexerState {
        LexerState::End
    }

    fn handle_numeric_eval(&mut self) -> LexerState {
        LexerState::End
    }
}