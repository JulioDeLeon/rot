use std::collections::HashMap;
use std::fmt;
use std::fmt::{Formatter, write};
use regex::Regex;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
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
    Bool,
    Char,

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
    StringLiteral,
    CharLiteral,
    MultiLnStringLiteral,
    RegexLiteral,

    Err(String)
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Comment => write!(f, "Comment"),
            Kind::IntLiteral => write!(f, "IntLiteral"),
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
            Kind::WhiteSpace => write!(f, "Space"),
            Kind::Elvis => write!(f, "Elvis"),
            Kind::Do => write!(f, "Do"),
            Kind::Def => write!(f, "Def"),
            Kind::DefStruct => write!(f, "DefStruct"),
            Kind::Return => write!(f, "Return"),
            Kind::Bool => write!(f, "Bool"),
            Kind::If => write!(f, "If"),
            Kind::Else => write!(f, "Else"),
            Kind::Static => write!(f, "Static"),
            Kind::For => write!(f, "For"),
            Kind::While => write!(f, "While"),
            Kind::Fn => write!(f, "Fn"),
            Kind::Public => write!(f, "Public"),
            Kind::Match => write!(f, "Match"),
            Kind::True => write!(f, "True"),
            Kind::False => write!(f, "False"),
            Kind::Double => write!(f, "Double"),
            Kind::DoubleLiteral => write!(f, "DoubleLiteral"),
            Kind::DefImpl => write!(f, "DefImpl"),
            Kind::U8 => write!(f, "U8"),
            Kind::U16 => write!(f, "U16"),
            Kind::U32 => write!(f, "U32"),
            Kind::U64 => write!(f, "U64"),
            Kind::U128 => write!(f, "U128"),
            Kind::I8 => write!(f, "I8"),
            Kind::I16 => write!(f, "I16"),
            Kind::I32 => write!(f, "I32"),
            Kind::I64 => write!(f, "I64"),
            Kind::I128 => write!(f, "I128"),
            Kind::Char => write!(f, "Char"),
            Kind::Null => write!(f, "Null"),
            Kind::Mutable => write!(f, "Mutable"),
            Kind::Float => write!(f, "Float"),
            Kind::USize => write!(f, "USize"),
            Kind::ISize => write!(f, "ISize"),
            Kind::Err(msg) => write!(f, "{}", format!("Kind::Error({})", msg)),
            _ => write!(f, "UNKNOWN CASE, NEED TO ADD PRINT HANDLE")
        }
    }
}

pub fn is_space(s: &str) -> bool {
    let space_regex: Regex = Regex::new(r"^\s+$").unwrap();
    return space_regex.is_match(s);
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    kind: Kind,
    lexeme: String,
    line_number: usize,
    line_position: usize,
}

impl Token {
    // char array with length?
    pub(crate) fn new(kind_t: Kind, text: String, line_num: usize, line_pos: usize) -> Token {
        Token {
            kind: kind_t,
            lexeme: text.parse().unwrap(),
            line_number: line_num,
            line_position: line_pos
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "token[kind: {}, lexeme: {}, line_number: {}, line_position: {}]",
               self.kind,
               self.lexeme,
               self.line_number,
               self.line_position
        )
    }
}

pub type SimpleDict = HashMap<String, Kind>;
pub type ComplexDict = Vec<(Regex, Kind)>;

pub fn build_complex_dictionary() -> ComplexDict {
    let mut ret: Vec<(Regex, Kind)> = Vec::new();
    ret.push((Regex::new(r"[ \t\r\f]+").unwrap(), Kind::WhiteSpace));
    ret.push((Regex::new(r"#.*\r?\n").unwrap(), Kind::Comment));
    ret.push((Regex::new(r#"^""".*"""$\r\n"#).unwrap(), Kind::Comment));
    ret.push((Regex::new(r"^[0-9]+$").unwrap(), Kind::IntLiteral));
    ret.push((Regex::new(r"^[0-9]+(\.[0-9]+)?$").unwrap(), Kind::DoubleLiteral));
    // ret.push((Regex::new(r"").unwrap(), Kind::Identifier));
    // advanced operators
    ret.push((Regex::new(r"\?:").unwrap(), Kind::Elvis));
    ret.push((Regex::new(r"\|\|").unwrap(), Kind::LogicalOr));
    ret.push((Regex::new(r"&&").unwrap(), Kind::LogicalAnd));
    ret.push((Regex::new(r"==").unwrap(), Kind::IsEqual));
    ret.push((Regex::new(r"!=").unwrap(), Kind::NotEqual));
    ret.push((Regex::new(r"-=").unwrap(), Kind::Increment));
    ret.push((Regex::new(r"\+=").unwrap(), Kind::Decrement));
    ret.push((Regex::new(r"<=").unwrap(), Kind::LessThanOrEqual));
    ret.push((Regex::new(r"\+=").unwrap(), Kind::GreaterThanOrEqual));

    ret.push((Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*").unwrap(), Kind::Identifier));
    return ret;
}

pub fn build_simple_dictionary() -> SimpleDict {
    //let mut ret: Vec<(String, Kind)> = Vec::new();
    let mut ret: HashMap<String, Kind> = HashMap::new();

    // generic symbols
    ret.insert("(".to_string() , Kind::LeftParen);
    ret.insert(")".to_string() , Kind::RightParen);
    ret.insert("[".to_string() , Kind::LeftBracket);
    ret.insert("]".to_string() , Kind::RightBracket);
    ret.insert("{".to_string() , Kind::LeftCurly);
    ret.insert("}".to_string() , Kind::RightCurly);
    ret.insert("<".to_string() , Kind:: LessThan);
    ret.insert(">".to_string() , Kind::GreaterThan);
    ret.insert("|".to_string() , Kind::Pipe);
    ret.insert("=".to_string() , Kind::Equal);
    ret.insert("+".to_string() , Kind::Plus);
    ret.insert("-".to_string() , Kind::Minus);
    ret.insert("*".to_string() , Kind::Asterisk);
    ret.insert("?".to_string() , Kind::Question);
    ret.insert("!".to_string() , Kind::Exclaim);
    ret.insert("&".to_string() , Kind::Ampersand);
    ret.insert("/".to_string() , Kind::Slash);
    ret.insert("#".to_string() , Kind::Hash);
    ret.insert(",".to_string() , Kind::Comma);
    ret.insert(".".to_string() , Kind::Dot);
    ret.insert("\"".to_string() , Kind::SingleQuote);
    ret.insert("\"".to_string() , Kind::DoubleQuote);
    ret.insert(":".to_string() , Kind::Colon);
    ret.insert(";".to_string() , Kind::Semicolon);

    // keywords
    ret.insert("def".to_string(), Kind::Def);
    ret.insert("defstruct".to_string(), Kind::DefStruct);
    ret.insert("defimpl".to_string(), Kind::DefImpl);
    ret.insert("do".to_string(), Kind::Do);
    ret.insert("fn".to_string(), Kind::Fn);
    ret.insert("end".to_string(), Kind::End);
    ret.insert("while".to_string(), Kind::While);
    ret.insert("for".to_string(), Kind::For);
    ret.insert("static".to_string(), Kind::Static);
    ret.insert("public".to_string(), Kind::Public);
    ret.insert("match".to_string(), Kind::Match);
    ret.insert("return".to_string(), Kind::Return);
    ret.insert("mut".to_string(), Kind::Mutable);
    ret.insert("if".to_string(), Kind::If);
    ret.insert("else".to_string(), Kind::Else);

    // types / values
    ret.insert("null".to_string(), Kind::Null);
    ret.insert("true".to_string(), Kind::True);
    ret.insert("false".to_string(), Kind::False);
    ret.insert("boolean".to_string(), Kind::Bool);
    ret.insert("double".to_string(), Kind::Double);
    ret.insert("float".to_string(), Kind::Float);
    ret.insert("char".to_string(), Kind::Char);
    ret.insert("string".to_string(), Kind::String);
    ret.insert("u8".to_string(), Kind::U8);
    ret.insert("u16".to_string(), Kind::U16);
    ret.insert("u32".to_string(), Kind::U32);
    ret.insert("u64".to_string(), Kind::U64);
    ret.insert("u128".to_string(), Kind::U128);
    ret.insert("i8".to_string(), Kind::I8);
    ret.insert("i16".to_string(), Kind::I16);
    ret.insert("i32".to_string(), Kind::I32);
    ret.insert("i64".to_string(), Kind::I64);
    ret.insert("i128".to_string(), Kind::I128);
    ret.insert("usize".to_string(), Kind::USize);
    ret.insert("isize".to_string(), Kind::ISize);

    return ret;
}

fn simple_eval_kind(dict: SimpleDict, input: String) -> Option<Kind> {
    return match dict.get(&input) {
        None => None,
        Some(kind_ptr) => Some(kind_ptr.clone())
    }
}

fn complex_eval_kind_h(expr: (Regex, Kind), input: String) -> Option<Kind> {
    let (regex, kind) = expr;
    return if regex.is_match(&input) {
        Some(kind)
    } else {
        None
    };
}

fn complex_eval_kind(dict: ComplexDict, input: String) -> Option<Kind> {
    for entry in dict {
        match complex_eval_kind_h(entry, input.clone()) {
            None => continue,
            Some(kind) => return Some(kind)
        }
    }

    return None;
}

pub fn find_kind(complex_dict: ComplexDict, simple_dictionary: SimpleDict, input: String) -> Option<Kind> {
    let ret = None;

    let mut check = simple_eval_kind(simple_dictionary, input.clone());
    if check != None {
        return check;
    }

    check = complex_eval_kind(complex_dict, input.clone());
    if check != None {
        return check
    }

    return ret;
}