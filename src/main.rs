extern crate phf;

mod compiler;

use compiler::*;
use crate::compiler::Keyword;

fn main() {
    let tokens: Vec<Token> = vec![
        Token::Keyword(Keyword::Logic(Logic::If))
    ];
    println!("Hello, world!");
}
