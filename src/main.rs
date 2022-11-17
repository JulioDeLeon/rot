extern crate core;

use std::{env, fs};

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.get(1) {
        None => {
            println!("need second argument");
            return;
        }
        Some(path) => path,
    };

    let content = fs::read(file_path).expect(&format!("could not read {}", file_path));
    let sample: Vec<char> = content.iter().map(|x| *x as char).collect();
    let mut lex = lexer::lexer::Lexer::new(sample);
    lex.parse();

    println!("Printing tokens");
    lex.tokens.iter().for_each(|tok| println!("{}", tok));
}
