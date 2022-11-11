use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use regex::Regex;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Kind {
    // keywords
    Identifier,
    Do,
    End,
    DefStruct,
    Def,
    DefImpl,
    Static,
    Return,
    Public, // expose?
    Mutable,
    Fn,
    Match,
    Else,
    If,
    For,
    While,

    // types
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
    ISize,
    USize,
    Double,
    Float,
    False,
    True,
    String,
    Null,
    Struct,

    // Ord()'s
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
    Question,
    Exclaim,
    Ampersand,
    Atom,
    WhiteSpace,

    // advance Operators
    LessThanOrEqual,
    GreaterThanOrEqual,
    IsEqual,
    NotEqual,
    LogicalAnd,
    LogicalOr,
    Increment,
    Decrement,
    Elvis,

    // literals
    IntLiteral,
    DoubleLiteral,

    Err(String)
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Comment => write!(f, "Comment"),
            Kind::IntLiteral => write!(f, "Number"),
            Kind::Identifier => write!(f, "Identifier"),
            Kind::LeftParen => write!(f, "LeftParen"),
            Kind::RightParen => write!(f, "RightParen"),
            Kind::LeftBracket => write!(f, "LeftBracket"),
            Kind::RightBracket => write!(f, "RightBracket"),
            Kind::LeftCurly => write!(f, "LeftCurly"),
            Kind::RightCurly => write!(f, "RightCurly"),
            Kind::LessThan => write!(f, "LessThan"),
            Kind::GreaterThan => write!(f, "GreaterThan"),
            Kind::Equal => write!(f, "Equal"),
            Kind::Plus => write!(f, "Plus"),
            Kind::Minus => write!(f, "Minus"),
            Kind::Asterisk => write!(f, "Asterisk"),
            Kind::Slash => write!(f, "Slash"),
            Kind::Hash => write!(f, "Hash"),
            Kind::Dot => write!(f, "Dot"),
            Kind::Comma => write!(f, "Comma"),
            Kind::Colon => write!(f, "Colon"),
            Kind::Semicolon => write!(f, "Semicolon"),
            Kind::SingleQuote => write!(f, "SingleQuote"),
            Kind::DoubleQuote => write!(f, "DoubleQuote"),
            Kind::Pipe => write!(f, "Pipe"),
            Kind::End => write!(f, "End"),
            Kind::Question => write!(f, "Question"),
            Kind::Exclaim => write!(f, "Exclaim"),
            Kind::Ampersand => write!(f, "Ampersand"),
            Kind::Atom => write!(f, "Atom"),
            Kind::WhiteSpace => write!(f, "Space"),
            _ => write!(f, "UNKNOWN CASE")
        }
    }
}

pub fn is_space(s: &str) -> bool {
    let space_regex: Regex = Regex::new(r"\s").unwrap();
    return space_regex.is_match(s);
}

pub fn is_digit(s: &str) -> bool {
    let number_regex: Regex = Regex::new(r"\d").unwrap();
    return number_regex.is_match(s);
}

fn is_identifier_char(s: &str) -> bool {
    let identifier_regex: Regex = Regex::new(r"^[a-zA-Z-1-9_]*$").unwrap();
    return identifier_regex.is_match(s);
}

#[derive(PartialEq, Debug)]
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

    pub fn to_string(&self) -> String {
        format!("Token[kind: {}, lexme: {}, position: {}]", self.kind, self.lexeme, self.position)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "token[kind: {}, lexeme: {}, position: {}]", self.kind, self.lexeme, self.position)
    }
}


pub fn build_dictionary() -> Vec<(Regex, Kind)> {
    let mut ret: Vec<(Regex, Kind)> = Vec::new();
    ret.push((Regex::new(r"[ \t\r\f]+").unwrap(), Kind::WhiteSpace));
    ret.push((Regex::new(r"#.*\r?\n").unwrap(), Kind::Comment));
    ret.push((Regex::new(r"\/\*(\*(?!\/)|[^*])*\*\/").unwrap(), Kind::Comment));
    ret.push((Regex::new(r"[0-9]+").unwrap(), Kind::IntLiteral));
    ret.push((Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*").unwrap(), Kind::Identifier));

    return ret;
}