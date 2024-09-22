use std::ops::Deref;
use crate::token::Token;

trait Node {
    fn token_literal(&self) -> String;
}

trait Statement: Node{
    fn statement_node(&self);
}

trait Expressions: Node{
    fn expression_node(&self);
}

struct Identifier {
    token: Token,
    value: String,
}

pub struct LetStatement<'let_statement> {
    token: Token,
    name: &'let_statement Identifier,
    value: dyn Expressions
}

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn token_literal(&self) -> String {
        match self.statements.first() {
            Some(el) => el.deref().token_literal(),
            None => "".to_string()
        }
    }
}

impl Identifier {
    fn expression_node(&self){}
    fn token_literal(&self) -> String {
        self.token.get_literal()
    }
}
