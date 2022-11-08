use lex::{Lexer, Token};

fn main() {
    println!("Hello, world!");
    
    let input = String::from("let a = 5;");
   // let mut l = Lexer::new(input.chars().collect());
    let mut l = Lexer::default();
    let lexed = l.lex(&input);

    for x in lexed.tokens() {
        // print tokens
        println!("token: {:?}, kind: {:?}", x.value(), x.kind().display_str());
    }
    // create lexer
    // read from file
    // print tokens
}
