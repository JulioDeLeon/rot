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
    Atom,
    Err(String)
}


pub struct Token {
    kind: Kind,
    lexeme: String,
    position: i32,
}

impl Token {
    // char array with length?
    pub(crate) fn new(kind: Kind, text: &str, pos: i32) -> Token {
        Token {
            kind,
            lexeme: text.parse().unwrap(),
            position: pos
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