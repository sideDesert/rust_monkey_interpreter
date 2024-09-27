#![allow(dead_code)]
use crate::token::Token;



pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait StatementTrait: Node {
    fn statement_node(&self);
}

pub trait Expression: Node + std::fmt::Debug {
    fn expression_node(&self);
}

#[derive(Debug)]
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
        Self {
            token: token.clone(),
            value: token.get_literal(),
        }
    }
    pub fn expression_node(&self) {}
}

#[derive(Debug)]
pub enum Statement {
    Let {
        token: Token,
        name : Option<Identifier>,
        value: Option<Box<dyn Expression>>,
    },
    Return {
        token: Token,
        value: Option<Box<dyn Expression>>
    }
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Self::Return{token:_, value: _} => "return".to_string(),
            Self::Let{token: _, name:_, value:_}  => "let".to_string()
        }
    }
}

impl StatementTrait for Statement {
    fn statement_node(&self) {}
}

impl Statement {
    pub fn new_let(token: Token) -> Self {
        Self::Let { token, name: None, value: None }
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: vec![]
        }
    }
    pub fn token_literal(&self) -> String {
        match self.statements.first() {
            Some(el) => el.token_literal(),
            None => "".to_string()
        }
    }
}

