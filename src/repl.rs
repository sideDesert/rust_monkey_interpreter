use std::io::{stdin, stdout, Write};

use crate::lexer::Lexer;
use crate::token::Token;
const PROMPT: &str = ">>";

pub fn start(){

    loop {
        let mut line = String::new();
        print!("{PROMPT} ");
        stdout().flush().unwrap();
        if stdin().read_line(&mut line).unwrap() == 0 {
            return;
        }
        let mut l = Lexer::new(&line);
        loop {
            let tok = l.next_token();
            if tok == Token::Eof {
                break;
            }
            println!("{:?}", tok);
        }
    }
}
