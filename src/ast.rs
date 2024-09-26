#![allow(dead_code)]

use std::ops::Deref;
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum StatementType {
    Let
}

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node{
    fn statement_node(&self);
    fn statement_type(&self) -> StatementType;
}

pub trait Expression: Node{
    fn expression_node(&self);
}


pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.get_literal()
    }
}
impl Identifier {
    pub fn new(token: Token) -> Self {
        Self{
            token: token.clone(),
            value: token.get_literal(),
        }
    }
    pub fn expression_node(&self){}
}


pub struct LetStatement {
    pub token: Token,
    pub name:  Option<Identifier>,
    pub value: Option<Box<dyn Expression>>
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        "let".to_string()
    }
}
impl Statement for LetStatement {
    fn statement_node(&self) {
        
    }

    fn statement_type(&self) -> StatementType {
        StatementType::Let
    }
}
impl LetStatement {
    pub fn new(token: Token)-> Self{
        Self {
            token: token.clone(),
            name:  None,
            value: None
        }
    }
}


pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: vec![]
        }
    }
    pub fn token_literal(&self) -> String {
        match self.statements.first() {
            Some(el) => el.deref().token_literal(),
            None => "".to_string()
        }
    }
}

