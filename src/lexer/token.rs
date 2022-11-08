enum TokenType {
    Err(Vec<char>)
}


struct Token {
    category: TokenType,
    text: string,
    position: int,
}

impl Token {
    fn new(tType: TokenType, text: string, pos: int) -> Token {
        Token {
            category: tType,
            text,
            position: pos
        }
    }
}