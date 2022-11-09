mod lexer;

fn main() {
    println!("Hello, world!");

    let sample = "some sample string";
    let mut lex = lexer::lexer::Lexer::new(sample.chars().collect());
    lex.parse();

    lex.tokens.iter().for_each(| tok | println!("{}", tok));
}
