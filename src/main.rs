mod lexer;
mod token;
use crate::lexer::Lexer;
fn main() {
    let raw_input = "let five = 5;
let ten = 10;
let add = fn(x,y){
x+y;
}

let result = add(five,ten)";
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
    println!("{:?}", l.next_token());
    println!("{:?}", l.next_token());
    println!("{:?}", l.next_token());
    println!("{:?}", l.next_token());
    println!("{:?}", l.next_token());
}
