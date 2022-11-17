use rot::lexer::lexer::Lexer;
use rot::lexer::token::Kind;

#[test]
fn test_complex_evaluation() {
    let sample = r#"def def
    test
    sammmple
    do
    end
    ?:
    ?
    check
    {
    +
    )
    return
    boolean
    bool
    if
    else
    static
    for
    while
    end
    fn
    do
    defimpl
    defstruct
    def
    public
    match
    mut
    if
    else
    null
    true
    false
    boolean
    double
    float
    u8
    i128
    usize
    isize
    30278420199
    9900.9
    "test string"
    # this is a comment
    'a'
    "\""
    "#;

    let mut lex = Lexer::new(sample.chars().collect());
    lex.parse();
    let toks = lex.tokens;

    assert_eq!(toks.is_empty(), false);
    assert_eq!(toks.len(), 47);
}

fn lexer_helper(term: String, expected_kind: Kind) {
    let mut lex = Lexer::new(term.chars().collect());
    lex.parse();
    let toks = lex.tokens;
    assert_eq!(toks.is_empty(), false);
    assert_eq!(toks.len(), 1);
    match toks.get(0) {
        Some(token) => {
            let t = token.clone();
            assert_eq!(t.kind, expected_kind);
            assert_eq!(t.lexeme, term);
        }
        None => {
            assert_eq!("Should not be here", "failing on purpose");
        }
    }
}

#[test]
fn test_keyword_lex() {
    lexer_helper("def".to_string(), Kind::Def)
}

#[test]
fn test_comment_lex() {
    lexer_helper("# some comment\n".to_string(), Kind::Comment);
    //lexer_helper("# some comment".to_string(), Kind::Comment);
    // TODO: need to do multiline comments
}

#[test]
fn test_numerical_lex() {
    lexer_helper("22223".to_string(), Kind::IntLiteral);
    lexer_helper("2222.22".to_string(), Kind::DoubleLiteral);
}

#[test]
fn test_string_lex() {
    lexer_helper("\"some string to evel\"".to_string(), Kind::StringLiteral);
    // TODO: need to do multiline strings
}

#[test]
fn test_symbol_lex() {
    lexer_helper("?:".to_string(), Kind::Elvis)
}

#[test]
fn test_regex_literal() {
    lexer_helper("r\"some regex\"".to_string(), Kind::RegexLiteral)
}
