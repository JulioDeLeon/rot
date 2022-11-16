/*
scan an string input?
parse input
return list of tokens

https://mohitkarekar.com/posts/pl/lexer/

look at scanner / lexer from code
 */

use crate::lexer::lexer::LexerState::*;
use crate::lexer::token::{
    build_complex_dictionary, build_simple_dictionary, find_kind, ComplexDict, SimpleDict,
};
use crate::lexer::token::{is_special_char, Token};
use core::fmt;

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
            return write!(f, "{}", format!("LexerState::Error({})", msg));
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
            _ => "unknown state",
        };
        write!(f, "{}", val)
    }
}

#[derive(Clone)]
pub struct Lexer {
    state: LexerState,
    pub tokens: Vec<Token>,
    input: Vec<char>,
    index: usize,
    line_number: usize,
    line_position: usize,
    complex_dict: ComplexDict,
    simple_dict: SimpleDict,
    buffer: String,
    last: Option<char>,
    curr: Option<char>,
}

impl fmt::Display for Lexer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let last_str = match self.last {
            None => "None".to_string(),
            Some(x) => x.to_string(),
        };

        let curr_str = match self.curr {
            None => "None".to_string(),
            Some(x) => x.to_string(),
        };

        let msg = format!(
            "lexer[
       state: {},
       tokens: <omit>,
       input: <omit>,
       index: {},
       line_number: {},
       line_position: {},
       <not printing dicts>,
       buffer: {},
       last: {},
       curr: {},
       ]",
            self.state,
            self.index,
            self.line_number,
            self.line_position,
            self.buffer,
            last_str,
            curr_str
        );
        write!(f, "{}", msg)
    }
}

impl Lexer {
    pub fn new(buff: Vec<char>) -> Lexer {
        Lexer {
            state: Start,
            tokens: Vec::new(),
            input: buff,
            index: 0,
            line_number: 0,
            line_position: 0,
            complex_dict: build_complex_dictionary(),
            simple_dict: build_simple_dictionary(),
            buffer: "".to_owned(),
            last: None::<char>,
            curr: None::<char>,
        }
    }

    fn peek(&self) -> Result<char, &str> {
        return if self.index < self.input.len() {
            Ok(self.input[self.index])
        } else {
            Err("indexing error")
        };
    }

    fn look_back(&mut self) -> Option<char> {
        return self.last.clone();
    }

    fn is_escaped(&self) -> bool {
        match self.last {
            Some(c) => {
                if c == '\\' {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn get(&mut self) -> Option<char> {
        return if self.index < self.input.len() {
            let ret = self.input[self.index];

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
                }
                None => (),
            }

            Some(ret)
        } else {
            None
        };
    }

    fn inc(&mut self) {
        self.index += 1;
    }

    pub fn parse(&mut self) {
        while self.state != End {
            let new_state = self.handle_state();
            if let Error(msg) = new_state {
                println!(
                    "Encountered an error while parsing: {}\nlexer : {}",
                    Error(msg),
                    self
                );
                break;
            }
            self.state = new_state
        }
    }

    pub fn reset(&mut self, buff: Vec<char>) {
        self.state = Start;
        self.input = buff;
        self.index = 0;
        self.line_position = 0;
        self.line_number = 0;
        // clear tokens?
    }

    fn handle_state(&mut self) -> LexerState {
        let snapshot_state: LexerState = self.state.clone();
        return match snapshot_state {
            Start => self.handle_start_state(),
            KeywordEval => self.handle_keyword_eval(),
            CommentEval => self.handle_comment_eval(),
            StringEval => self.handle_string_eval(),
            MultiLnStringEval => self.handle_multilnstring_eval(),
            MultiLnCommentEval => self.handle_multilncomment_eval(),
            CharEval => self.handle_char_eval(),
            MaybeRegexEval => self.handle_maybe_regex(),
            RegexEval => self.handle_regex_eval(),
            NumericEval => self.handle_numeric_eval(),
            End => End,
            Error(msg) => {
                println!("Lexer in error state: {}", Error(msg));
                End
            }
        };
    }

    fn handle_buffer(&mut self) {
        if let Some(t_kind) = find_kind(
            self.complex_dict.clone(),
            self.simple_dict.clone(),
            self.buffer.clone(),
        ) {
            let t_token = Token::new(
                t_kind,
                self.buffer.clone(),
                self.line_number,
                self.line_position - self.buffer.len(),
            );
            self.tokens.push(t_token);
        } else {
            println!("was not able to derive kind from buffer: {}", self.buffer);
        }
    }

    fn flush_buffer(&mut self) {
        if !self.buffer.is_empty() {
            self.handle_buffer()
        }
        self.buffer = "".to_string()
    }

    fn handle_general_complex_case(&mut self, x: char) -> LexerState {
        if x.is_whitespace() {
            self.flush_buffer();
            Start
        } else if x.is_alphabetic() || is_special_char(x) {
            self.buffer.push(x);
            KeywordEval
        } else if x.is_numeric() {
            self.buffer.push(x);
            NumericEval
        } else {
            Error("could not determine case".to_string())
        }
    }

    fn handle_start_state_simple_case(&mut self, x: char) -> LexerState {
        match x {
            'r' => {
                self.buffer.push(x);
                MaybeRegexEval
            }
            '"' => {
                self.buffer.push(x);
                StringEval
            }
            '#' => {
                self.buffer.push(x);
                CommentEval
            }
            '\'' => {
                self.buffer.push(x);
                CharEval
            }
            _ => self.handle_general_complex_case(x.clone()),
        }
    }

    fn handle_start_state(&mut self) -> LexerState {
        let check = self.get();

        return match check {
            Some(c) => self.handle_start_state_simple_case(c.clone()),
            None => End,
        };
    }

    fn handle_comment_eval(&mut self) -> LexerState {
        let check = self.get();
        match check {
            Some(c) => {
                if c == '\n' {
                    self.buffer.push(c);
                    self.flush_buffer();
                    Start
                } else {
                    self.buffer.push(c);
                    CommentEval
                }
            }
            _ => {
                self.flush_buffer();
                Start
            }
        }
    }

    fn handle_string_eval(&mut self) -> LexerState {
        let check = self.get();

        match check {
            Some(c) => {
                if c == '"' {
                    self.handle_escaped_delim(c, StringEval, Start)
                } else if c == '\n' {
                    Error("found newline in possible string".to_string())
                } else {
                    self.buffer.push(c);
                    StringEval
                }
            }
            _ => {
                self.flush_buffer();
                Start
            }
        }
    }

    fn handle_multilnstring_eval(&mut self) -> LexerState {
        LexerState::Error("multilnstring state TODO".to_string())
    }

    fn handle_multilncomment_eval(&mut self) -> LexerState {
        LexerState::Error("multilncomment state TODO".to_string())
    }

    fn handle_keyword_eval(&mut self) -> LexerState {
        let check = self.get();
        match check {
            Some(c) => {
                if c.is_alphanumeric() || is_special_char(c) {
                    self.buffer.push(c);
                    KeywordEval
                } else if c.is_whitespace() {
                    self.flush_buffer();
                    Start
                } else {
                    self.buffer.push(c);
                    Error(format!("issue lexing {}", self.buffer))
                }
            }
            _ => {
                self.flush_buffer();
                Start
            }
        }
    }

    fn handle_maybe_regex(&mut self) -> LexerState {
        let check = self.get();
        match check {
            Some(c) => {
                if c.is_whitespace() {
                    self.flush_buffer();
                    Start
                } else if c == '"' {
                    self.buffer.push(c);
                    RegexEval
                } else {
                    self.buffer.push(c);
                    KeywordEval
                }
            }
            _ => {
                self.flush_buffer();
                Start
            }
        }
    }

    fn handle_escaped_delim(
        &mut self,
        x: char,
        escaped_state: LexerState,
        non_escaped_state: LexerState,
    ) -> LexerState {
        if self.is_escaped() {
            self.buffer.push(x);
            escaped_state
        } else {
            self.buffer.push(x);
            self.flush_buffer();
            non_escaped_state
        }
    }

    fn handle_regex_eval(&mut self) -> LexerState {
        let check = self.get();

        match check {
            Some(c) => {
                if c == '"' {
                    self.handle_escaped_delim(c, RegexEval, Start)
                } else if c == '\n' {
                    Error("regex eval has seen a new line in regex expression".to_string())
                } else {
                    self.buffer.push(c);
                    RegexEval
                }
            }
            _ => {
                self.flush_buffer();
                Start
            }
        }
    }

    fn handle_char_eval(&mut self) -> LexerState {
        let check = self.get();

        match check {
            Some(c) => {
                if c == '\'' {
                    self.handle_escaped_delim(c, CharEval, Start)
                } else if c == '\n' {
                    Error("found newline in possible char".to_string())
                } else {
                    self.buffer.push(c);
                    CharEval
                }
            }
            _ => {
                self.flush_buffer();
                Start
            }
        }
    }

    fn handle_numeric_eval(&mut self) -> LexerState {
        let check = self.get();

        match check {
            Some(c) => {
                if c.is_numeric() || c == '.' {
                    self.buffer.push(c);
                    NumericEval
                } else if c.is_whitespace() {
                    self.flush_buffer();
                    Start
                } else {
                    self.buffer.push(c);
                    Error(format!("issue lexing {}", self.buffer))
                }
            }
            _ => {
                self.flush_buffer();
                Start
            }
        }
    }
}
