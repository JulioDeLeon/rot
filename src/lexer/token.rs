enum TokenType {
    Err(Vec<char>)
}


struct Token {
    category: TokenType,
    text: String,
    position: i32,
}

impl Token {
    fn new(tType: TokenType, text: String, pos: i32) -> Token {
        Token {
            category: tType,
            text,
            position: pos
        }
    }
}