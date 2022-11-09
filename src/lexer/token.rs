use std::fmt;
use regex::Regex;

#[derive(PartialEq)]
pub enum Kind {
    Number,
    Identifier,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftCurly,
    RightCurly,
    LessThan,
    GreaterThan,
    Equal,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Hash,
    Dot,
    Comma,
    Colon,
    Semicolon,
    SingleQuote,
    DoubleQuote,
    Comment,
    Pipe,
    End,
    Question,
    Exclaim,
    Ampersand,
    Atom,
    Space,
    Err(String)
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub fn is_space(c: char) -> bool {
    let space_regex: Regex = Regex::new(r"\s").unwrap();
    return space_regex.is_match(&*c.to_string());
}

pub fn is_digit(c: char) -> bool {
    let number_regex: Regex = Regex::new(r"\d").unwrap();
    return number_regex.is_match(&*c.to_string());
}

fn is_identifier_char(c: char) -> bool {
    let identifier_regex: Regex = Regex::new(r"^[a-zA-Z-1-9_]*$").unwrap();
    return identifier_regex.is_match(&*c.to_string());
}

fn char_to_kind(c: char) -> Kind {
    match c {
        '(' => Kind::LeftParen,
        ')' => Kind::RightParen,
        '[' => Kind::LeftBracket,
        ']' => Kind::RightBracket,
        '{' => Kind::LeftCurly,
        '}' => Kind::RightCurly,
        '<' => Kind:: LessThan,
        '>' => Kind::GreaterThan,
        '|' => Kind::Pipe,
        '=' => Kind::Equal,
        '+' => Kind::Plus,
        '-' => Kind::Minus,
        '*' => Kind::Asterisk,
        '?' => Kind::Question,
        '!' => Kind::Exclaim,
        '&' => Kind::Ampersand,
        '/' => Kind::Slash,
        '#' => Kind::Hash,
        ',' => Kind::Comma,
        '.' => Kind::Dot,
        '\'' => Kind::SingleQuote,
        '"' => Kind::DoubleQuote,
        '\0' => Kind::End,
        _ => {
            if is_space(c) { Kind::Space }
            else if is_digit(c) { Kind::Number }
            else if is_identifier_char(c) { Kind::Identifier }
            else { Kind::Err("Error unrecognized: {c}".parse().unwrap()) }
        },
    }
}


pub struct Token {
    kind: Kind,
    lexeme: String,
    position: usize,
}

impl Token {
    // char array with length?
    pub(crate) fn new(kind: Kind, text: &str, pos: usize) -> Token {
        Token {
            kind,
            lexeme: text.parse().unwrap(),
            position: pos,
        }
    }

    fn is(&self, kind: Kind) -> bool {
        return self.kind == kind;
    }

    fn is_not(&self, kind: Kind) -> bool {
        return self.kind != kind;
    }

    fn is_one_of(&self, kinds: Vec<Kind>) -> bool {
        let mut ret: bool = false;

        for x in kinds {
           ret = self.is(x);
           if ret {
               break;
           }
        }

        return ret;
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token[kind: {}, lexme: {}, position: {}]", self.kind, self.lexeme, self.position)
    }
}