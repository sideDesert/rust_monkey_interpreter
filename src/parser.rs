#![allow(dead_code)]

use crate::ast::{Identifier, LetStatement, Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser<'parser> {
    pub lexer: &'parser mut Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<'parser> Parser<'parser> {
    pub fn new(lexer: &'parser mut Lexer) -> Self {
        let mut p = Self {
            cur_token: None,
            peek_token: None,
            lexer,
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program::new();
        while self.cur_token != Some(Token::Eof) {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        if let Some(cur_tok) = self.cur_token.as_ref() {
            match cur_tok {
                Token::Let => {
                    let stmt = self.parse_let_statement().unwrap();
                    let let_stmt: Box<dyn Statement> = Box::new(stmt);
                    Some(let_stmt)
                }
                _ => None
            }
        } else {
            None
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let mut stmt = LetStatement::new(self.cur_token.clone()?);

        if !matches!(self.cur_token, Some(Token::Ident(_))){
            return None
        }

        stmt.name = Some(Identifier::new(self.cur_token.clone()?));
        if !self.expect_peek(Token::Assign){
            return None
        }

        while !self.cur_token_is(Token::Semicolon) {
            self.next_token();
        }
        Some(stmt)
    }

    fn cur_token_is(&self, t: Token) -> bool {
        match self.cur_token.as_ref() {
            None => false,
            Some(tok) => *tok == t
        }
    }

    fn peek_token_is(&self, t: Token) -> bool {
        match self.peek_token.as_ref() {
            None => false,
            Some(tok) => *tok == t
        }
    }
    
    fn expect_peek(&mut self, t: Token) -> bool {
        if self.peek_token_is(t){
            self.next_token();
            return true;
        }     
        false
    }
}

#[cfg(test)]
mod test {
    use std::{any::Any,ops::Deref};

    use crate::{ast::{LetStatement, Node, Statement, StatementType}, lexer::Lexer};

    use super::Parser;

    //fn test_let_statements() {
    //    let test = String::from(
    //        "let x = 5;
    //let //y = 10;
    //let //foobar = 83838;",
    //    );

    //    let mut lexer = Lexer::new(&test);
    //    let mut parser = Parser::new(&mut lexer);
    //    let program = parser.parse_program();

    //    match program {
    //        None => eprintln!("parse_program() returned None"),
    //        Some(p) => {
    //            if p.statements.len() != 3 {
    //                eprintln!("program.statements does not contain 3 statements.");
    //            }
    //            let tests= ["x", "y", "foobar"];
    //            for (i, t) in tests.iter().enumerate() {
    //                let stmt  = p.statements.get(i).unwrap().deref();
    //                if !test_let_statement(Box::new(stmt), t) {
    //                    return;
    //                }
    //            }
    //        }
    //    }
    //}

    //fn test_let_statement<'test>(s: Box<&dyn Statement>, name: &str) -> bool
    //{
    //    let a = s.token_literal();
    //    if a != "let" {
    //        eprintln!("s.token_literal() not 'let'. got={a}");
    //        return false;
    //    }

    //    if s.statement_type() != StatementType::Let {
    //        eprintln!("s.statement_type() not 'let'.");
    //        return false;
    //    }

    //    let l: Box<dyn Any> = s;
    //    if let Ok(let_stmt) = l.downcast::<LetStatement>() {
    //        if let_stmt.name.value != name {
    //            eprintln!("let_stmtm.name.value is not {name}, got {}", let_stmt.name.value);
    //            return false;
    //        }

    //        if let_stmt.name.token_literal() != name {
    //            eprintln!("let_stmtm.name.token_literal() is not {name}, got {}", let_stmt.name.token_literal());
    //            return false;
    //        }
    //    }
    //    true
    //}
}
