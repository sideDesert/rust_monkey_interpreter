#![allow(dead_code)]
use crate::token::{Token, lookup_ident};

#[derive(Default, Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
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

    fn is_letter(ch:char) -> bool {
        if ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch == '_' {
            return true;
        }
        false
    }

    fn is_digit(ch:char) -> bool{
        ch.is_numeric()
    }
    
    pub fn peek_char(&self) -> char{
        self.input.chars().nth(self.read_position).unwrap_or('\0')
    }

    pub fn read_char(&mut self){
        let read_pos = self.read_position;
        if read_pos > self.input.len() {
            self.ch = None
        } else {
            self.ch = self.input.chars().nth(read_pos);
        }

        self.position = read_pos;
        self.read_position += 1;
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if ch != ' ' && ch != '\t' && ch != '\n' && ch != '\r' {
                return;
            } 
            self.read_char();
        }
    }
    pub fn next_token(&mut self) -> Token{
        self.skip_whitespace();
        let token = match self.ch {
            Some('-') => Token::Minus, 
            Some('+') => Token::Plus,
            Some(';') => Token::Semicolon, 
            Some('(') => Token::Lparen, 
            Some(')') => Token::Rparen,
            Some('{') => Token::Lbrace,
            Some('}') => Token::Rbrace,
            Some(',') => Token::Comma,
            Some('<') => Token::Lt,
            Some('>') => Token::Gt,
            Some('*') => Token::Asterisk,
            Some('/') => Token::Slash,
            Some('=') if self.peek_char() == '=' => {
                self.read_char();
                Token::Eq
            }
            Some('=') => Token::Assign,
            Some('!') if self.peek_char() == '=' => {
                self.read_char();
                Token::NotEq
            }
            Some('!') => Token::Bang,
            Some(char) if Self::is_letter(char)  => {
                    return lookup_ident(self.read_identifier())
            },
            Some(char) if Self::is_digit(char)  => {
                    return Token::Int(self.read_number().to_string());
            },
            Some(_char) => Token::Illegal,
            None => Token::Eof,
        };
        self.read_char();
        token
    }

    fn read_number(&mut self) -> &str {
        let start = self.position;
        while let Some(ch) = self.ch {
            if !Self::is_digit(ch) {
                break;
            }
            self.read_char();
        }
        let end = self.position;
        &self.input[start..end]
    }

    fn read_identifier(&mut self) -> &str {
        let start = self.position;
        while let Some(curr_char) = self.ch {
            if !Self::is_letter(curr_char) {
                break;
            }
            self.read_char();
        }         
        let end = self.position;
        &self.input[start..end]
    }
}

#[cfg(test)]
mod test {
    use std::borrow::BorrowMut;
    use crate::token::Token::*;
    use crate::lexer::Lexer;
    #[test]
    fn test_next_token(){
        let raw_input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;";
        let input = String::from(raw_input);
        let mut l = Lexer::new(&input);
        let tests = vec![
            Let,
            Ident("five".to_string()),
            Assign,
            Int("5".to_string()),
            Semicolon,
            Let,
            Ident("ten".to_string()),
            Assign,
            Int("10".to_string()),
            Semicolon,
            Let,
            Ident("add".to_string()),
            Assign,
            Function,
            Lparen,
            Ident("x".to_string()),
            Comma,
            Ident("y".to_string()),
            Rparen,
            Lbrace,
            Ident("x".to_string()),
            Plus,
            Ident("y".to_string()),
            Semicolon,
            Rbrace,
            Semicolon,
            Let,
            Ident("result".to_string()),
            Assign,
            Ident("add".to_string()),
            Lparen,
            Ident("five".to_string()),
            Comma,
            Ident("ten".to_string()),
            Rparen,
            Semicolon,
            Bang,
            Minus,
            Slash,
            Asterisk,
            Int("5".to_string()),
            Semicolon,
            Int("5".to_string()),
            Lt,
            Int("10".to_string()),
            Gt,
            Int("5".to_string()),
            Semicolon,
        ];

        for test in tests {
            let tok = l.borrow_mut().next_token();
            assert_eq!(tok, test);
        }
    }
}
