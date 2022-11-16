mod lexer;

fn main() {
    println!("Hello, world!");

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
    "#;
    let mut lex = lexer::lexer::Lexer::new(sample.chars().collect());
    lex.parse();

    lex.tokens.iter().for_each(|tok| println!("{}", tok));
}
