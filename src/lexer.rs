#![allow(dead_code)]
use crate::token::{TokenType, Token};

#[derive(Default, Debug)]
pub struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    ch: Option<char>
}

impl Lexer {
    pub fn new(input: &str) -> Self{
        let mut l = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: None
        };
        l.read_char();
        l
    }

    pub fn read_char(&mut self){
        let read_pos = self.read_position;
        if read_pos > self.input.len() as i32 {
            self.ch = None
        } else {
            self.ch = self.input.chars().nth(read_pos as usize);
        }

        self.position = read_pos;
        self.read_position += 1;
    }


    pub fn next_token(&mut self) -> Token{
        let token = match self.ch{
            Some('=') => Token::new(TokenType::Assign, &self.ch.unwrap().to_string()),
            Some('+') => Token::new(TokenType::Plus, &self.ch.unwrap().to_string()),
            Some(';') => Token::new(TokenType::Semicolon, &self.ch.unwrap().to_string()),
            Some('(') => Token::new(TokenType::Lparen, &self.ch.unwrap().to_string()),
            Some(')') => Token::new(TokenType::Rparen, &self.ch.unwrap().to_string()),
            Some('{') => Token::new(TokenType::Lbrace, &self.ch.unwrap().to_string()),
            Some('}') => Token::new(TokenType::Rbrace, &self.ch.unwrap().to_string()),
            Some(',') => Token::new(TokenType::Comma, &self.ch.unwrap().to_string()),
            None => Token::new(TokenType::Eof, ""),
            _ => Token::new(TokenType::Eof, "")
        };
        self.read_char();
        token
    }
}

#[cfg(test)]
mod test {
    use std::borrow::BorrowMut;

    use crate::token::{TokenType, Token};
    use crate::lexer::Lexer;
    #[test]
    fn test_next_token(){
        let input = String::from("=+(){},;");
        let mut l = Lexer::new(&input);
        let tests = vec![
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Plus, "+"),
            Token::new(TokenType::Lparen, "("),
            Token::new(TokenType::Rparen, ")"),
            Token::new(TokenType::Lbrace, "{"),
            Token::new(TokenType::Rbrace, "}"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Eof, ""),
        ];

        for test in tests {
            let tok = l.borrow_mut().next_token();
            assert_eq!(tok, test);
        }
    }
}
