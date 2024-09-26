#![allow(dead_code)]

#[derive(Debug, PartialEq, Clone)]
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

impl Token {
    pub fn get_literal(&self) -> String {
        match self {
            Token::Illegal => "".to_string(),
            Token::Eof => "".to_string(),
            Token::Assign => "=".to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Bang => "!".to_string(),
            Token::Asterisk => "*".to_string(),
            Token::Slash => "/".to_string(),
            Token::Lt => "<".to_string(),
            Token::Gt => ">".to_string(),
            Token::Comma => ",".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::Lparen => "{".to_string(),
            Token::Rparen => "}".to_string(),
            Token::Lbrace => "(".to_string(),
            Token::Rbrace => ")".to_string(),
            Token::Let => "let".to_string(),
            Token::Ident(identity) => identity.to_string(),
            Token::Int(integer) => integer.to_string(),
            Token::Function => "fn".to_string(), 
            Token::True => "true".to_string(),
            Token::False => "false".to_string(),
            Token::If => "if".to_string(),
            Token::Else => "else".to_string(),
            Token::Return => "return".to_string(),
            Token::Eq => "==".to_string(),
            Token::NotEq => "!=".to_string(),
        }
    }
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

