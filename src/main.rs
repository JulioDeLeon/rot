mod lexer;

fn main() {
    println!("Printing tokens");
    let sample = r#"
    r"some regex"
    "#;
    let mut lex = lexer::lexer::Lexer::new(sample.chars().collect());
    lex.parse();

    lex.tokens.iter().for_each(|tok| println!("{}", tok));
}
