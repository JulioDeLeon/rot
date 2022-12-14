use regex::RegexSet;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

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
    Type,
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
    Arrow,

    // literals
    IntLiteral,
    DoubleLiteral,
    StringLiteral,
    CharLiteral,
    MultiLnStringLiteral,
    RegexLiteral,
    Err(String),
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
            Kind::WhiteSpace => write!(f, "WhiteSpace"),
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
            Kind::StringLiteral => write!(f, "StringLiteral"),
            Kind::CharLiteral => write!(f, "CharLiteral"),
            Kind::RegexLiteral => write!(f, "RegexLiteral"),
            Kind::MultiLnStringLiteral => write!(f, "MultiLnStringLiteral"),
            Kind::IsEqual => write!(f, "IsEqual"),
            Kind::NotEqual => write!(f, "NotEqual"),
            Kind::LogicalAnd => write!(f, "LogicalAnd"),
            Kind::LogicalOr => write!(f, "LogicalOr"),
            Kind::Increment => write!(f, "Increment"),
            Kind::Decrement => write!(f, "Decrement"),
            Kind::LessThanOrEqual => write!(f, "LessThanOrEqual"),
            Kind::GreaterThanOrEqual => write!(f, "GreaterThanOrEqual"),
            Kind::Type => write!(f, "Type"),
            Kind::Arrow => write!(f, "Arrow"),
            Kind::Err(msg) => write!(f, "{}", format!("Kind::Error({})", msg)),
            _ => write!(f, "UNKNOWN CASE, NEED TO ADD PRINT HANDLE"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub kind: Kind,
    pub lexeme: String,
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
            line_position: line_pos,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "token[kind: {}, lexeme: {}, line_number: {}, line_position: {}]",
            self.kind,
            self.lexeme.trim(),
            self.line_number,
            self.line_position
        )
    }
}

pub type SimpleDict = HashMap<String, Kind>;
pub type ComplexDict = (HashMap<usize, Kind>, RegexSet);

pub fn build_complex_dictionary() -> ComplexDict {
    let mut links: Vec<(String, Kind)> = Vec::new();
    links.push((r"^[ \t\r\f]+$".to_string(), Kind::WhiteSpace));
    links.push((r"^#.*\n$".to_string(), Kind::Comment));
    links.push((r#"^""".*"""$\r\n"#.to_string(), Kind::MultiLnStringLiteral));
    links.push((r"^[0-9]+$".to_string(), Kind::IntLiteral));
    links.push((r#"^".*"$"#.to_string(), Kind::StringLiteral));
    links.push((r#"^'.*'$"#.to_string(), Kind::CharLiteral));
    links.push((r#"^r".*"$"#.to_string(), Kind::RegexLiteral));
    links.push((r"^[0-9]+(\.[0-9]+)?$".to_string(), Kind::DoubleLiteral));
    links.push((r"^\?:$".to_string(), Kind::Elvis));
    links.push((r"^\|\|$".to_string(), Kind::LogicalOr));
    links.push((r"^&&$".to_string(), Kind::LogicalAnd));
    links.push((r"^==$".to_string(), Kind::IsEqual));
    links.push((r"^!=$".to_string(), Kind::NotEqual));
    links.push((r"^-=$".to_string(), Kind::Increment));
    links.push((r"^\+=$".to_string(), Kind::Decrement));
    links.push((r"^<=$".to_string(), Kind::LessThanOrEqual));
    links.push((r"^\+=$".to_string(), Kind::GreaterThanOrEqual));
    links.push((r"^\->$".to_string(), Kind::Arrow));
    links.push((r"[a-zA-Z_][a-zA-Z0-9_]*".to_string(), Kind::Identifier));

    let mut dict: HashMap<usize, Kind> = HashMap::new();
    let mut patterns: Vec<String> = Vec::new();
    for (i, x) in links.iter().enumerate() {
        let (pattern, kind) = x;
        dict.insert(i, kind.clone());
        patterns.push(pattern.to_string())
    }
    let set = RegexSet::new(patterns).unwrap();
    (dict, set)
}

pub fn is_special_char(x: char) -> bool {
    let haystack = r#"!*)(][}{\|:?/,.;-+<>&="#;
    haystack.contains(x)
}

pub fn build_simple_dictionary() -> SimpleDict {
    //let mut ret: Vec<(String, Kind)> = Vec::new();
    let mut ret: HashMap<String, Kind> = HashMap::new();

    // generic symbols
    ret.insert("(".to_string(), Kind::LeftParen);
    ret.insert(")".to_string(), Kind::RightParen);
    ret.insert("[".to_string(), Kind::LeftBracket);
    ret.insert("]".to_string(), Kind::RightBracket);
    ret.insert("{".to_string(), Kind::LeftCurly);
    ret.insert("}".to_string(), Kind::RightCurly);
    ret.insert("<".to_string(), Kind::LessThan);
    ret.insert(">".to_string(), Kind::GreaterThan);
    ret.insert("|".to_string(), Kind::Pipe);
    ret.insert("=".to_string(), Kind::Equal);
    ret.insert("+".to_string(), Kind::Plus);
    ret.insert("-".to_string(), Kind::Minus);
    ret.insert("*".to_string(), Kind::Asterisk);
    ret.insert("?".to_string(), Kind::Question);
    ret.insert("!".to_string(), Kind::Exclaim);
    ret.insert("&".to_string(), Kind::Ampersand);
    ret.insert("/".to_string(), Kind::Slash);
    ret.insert("#".to_string(), Kind::Hash);
    ret.insert(",".to_string(), Kind::Comma);
    ret.insert(".".to_string(), Kind::Dot);
    ret.insert("\"".to_string(), Kind::SingleQuote);
    ret.insert("\"".to_string(), Kind::DoubleQuote);
    ret.insert(":".to_string(), Kind::Colon);
    ret.insert(";".to_string(), Kind::Semicolon);

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
    ret.insert("type".to_string(), Kind::Type);

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
        Some(kind_ptr) => Some(kind_ptr.clone()),
    };
}

fn complex_eval_kind(dict: ComplexDict, input: String) -> Option<Kind> {
    let (translator, reg) = dict;
    let matches: Vec<_> = reg.matches(&input).into_iter().collect();
    if matches.is_empty() {
        None
    } else {
        let first = matches[0];
        let kind_opt = translator.get(&first);
        match kind_opt {
            Some(kind) => Some(kind.clone()),
            None => None,
        }
    }
}

pub fn find_kind(
    complex_dict: ComplexDict,
    simple_dictionary: SimpleDict,
    input: String,
) -> Option<Kind> {
    let ret = None;

    let mut check = simple_eval_kind(simple_dictionary, input.clone());
    if check != None {
        return check;
    }

    check = complex_eval_kind(complex_dict, input.clone());
    if check != None {
        return check;
    }

    return ret;
}
