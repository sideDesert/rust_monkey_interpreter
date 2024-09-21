mod lexer;
mod token;
use crate::lexer::Lexer;
fn main() {
    let raw_input = "if (5 != x){ return a; }";

    let mut l = Lexer::new(raw_input);
    println!("{:?}", l.next_token());
    println!("{:?}", l.next_token());
    println!("{:?}", l.next_token());
    println!("{:?}", l.next_token());
    println!("{:?}", l.next_token());
    println!("{:?}", l.next_token());
    println!("{:?}", l.next_token());
    println!("{:?}", l.next_token());
    println!("{:?}", l.next_token());
    println!("{:?}", l.next_token());
    println!("{:?}", l.next_token());
    println!("{:?}", l.next_token());
}
