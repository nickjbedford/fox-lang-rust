extern crate phf;

mod compiler;

use compiler::*;

fn main() {
    let tokens: Vec<Token> = vec![
        Token::Keyword(Keyword::Logical(Logical::If))
    ];
    println!("Hello, world!");
}
