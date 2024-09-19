mod lexer;
mod token;
use crate::lexer::Lexer;
fn main() {
    let mut l = Lexer::new("+={}");
    println!("{:?}", l);
}
