#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function(String),
    Let(String),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token: TokenType,
    literal: String
}

impl Token {
    pub fn new(token: TokenType, literal: &str) -> Self {
        Token {
            token,
            literal : literal.to_string()
        }
    }
}


