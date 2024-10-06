#![allow(dead_code)]
use crate::token::Token;
use std::fmt::{Display, Debug};

pub trait Node: Display {
    fn token_literal(&self) -> String;
}

pub trait StatementTrait: Node {
    fn statement_node(&self);
}

pub trait Expression: Node + Debug {
    fn expression_node(&self);
}


#[derive(Debug)]
pub enum Statement {
    Let {
        token: Token,
        name : Option<Identifier>,
        value: Option<Box<dyn Expression>>
    },
    Return {
        token: Token,
        value: Option<Box<dyn Expression>>
    },
    Expression {
        token: Token,
        expression: Option<Box<dyn Expression>>
    }
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Self::Return{token:_, value: _} => "return".to_string(),
            Self::Let{token: _, name:_, value:_}  => "let".to_string(),
            Self::Expression { token, expression: _ } => token.get_literal().to_string(),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression { token:_, expression } => {
                let mut str = String::from("");
                if let Some(exp) = expression {
                    str = format!("{}", exp);
                }
                write!(f, "{}", str)
            },

            Self::Let { token, name, value  } => {
                let mut str = String::new();
                if let Some(name) = name {
                    str = format!("{} {}",&token.get_literal(), &name.token_literal());
                    if let Some(val) = value {
                        str = format!("{} {} = {};",&token.get_literal(), &name, &val);
                    }
                }
                write!(f, "{}", str)
            },

            Self::Return { token, value } => {
                let mut str = String::from("return;");
                if let Some(val) = value {
                    str = format!("{} {};", &token.get_literal(), &val);
                }
                write!(f, "{}", str)
            }
        }
    }
}

impl StatementTrait for Statement {
    fn statement_node(&self) {}
}

impl Statement {
    pub fn new(token: Token) -> Option<Self> {
        match token {
            Token::Let => Some(Self::Let { token, name: None, value: None }),
            Token::Return => Some(Self::Return { token, value: None }),
            _ => None
        }
    }
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

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.value)
    }
}

impl Identifier {
    pub fn new(token: Token) -> Self {
        Self {
            token: token.clone(),
            value: token.get_literal(),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in self.statements.iter() {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
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

