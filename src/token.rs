#![allow(dead_code)]


#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
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
    True,
    False,
    If,
    Else,
    Return,
    Eq,
    NotEq,
}

pub fn lookup_ident(literal: &str) -> Token {
    match literal {
        "fn" => Token::Function,
        "let" => Token::Let,
        "return" => Token::Return,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        word => Token::Ident(word.to_string())
    }
}

