#![allow(dead_code)]


#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Let,
    Ident(String),
    Int(String),
    Function, 
}

pub fn lookup_ident(literal: &str) -> Token {
    match literal {
        "fn" => Token::Function,
        "let" => Token::Let,
        word => Token::Ident(word.to_string())
    }
}

