#![allow(dead_code)]
use crate::token::Token;
use std::{any::Any, fmt::{Debug, Display}};

pub trait Node: Display {
    fn token_literal(&self) -> String;
}

pub trait StatementTrait: Node {
    fn statement_node(&self);
}

pub trait Expression: Node + Debug {
    fn expression_node(&self);
    fn as_any(&self) -> &dyn Any;
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
impl Expression for Identifier {
    fn expression_node(&self) {
        
    }

    fn as_any(&self) -> &dyn Any {
        self
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
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i32,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.get_literal()
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {
        
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.value)
    }
}

impl IntegerLiteral {
    pub fn new(token: Token, value: i32) -> Self {
        Self {
            token: token.clone(),
            value
        }
    }
}

// Prefix Expression
#[derive(Debug)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<dyn Expression>
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}


    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"({}{})", self.operator, self.right)
    }
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.get_literal()
    }
}

// Infix Expression
#[derive(Debug)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: String,
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}


    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"({} {} {})", self.left, self.operator, self.right)
    }
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.get_literal()
    }
}

// Program
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

