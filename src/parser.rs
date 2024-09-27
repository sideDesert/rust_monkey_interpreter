#![allow(dead_code)]

use crate::ast::{Identifier, Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser<'parser> {
    pub lexer: &'parser mut Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
}

impl<'parser> Parser<'parser> {
    pub fn new(lexer: &'parser mut Lexer) -> Self {
        let mut p = Self {
            cur_token: None,
            peek_token: None,
            errors: vec![],
            lexer,
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        while self.cur_token != Some(Token::Eof) {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        program
    }

    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }
        
    fn peek_error(&mut self, t: Token) {
        let msg = format!("expected next token to be {}, got {:?} instead", &t.get_literal(), self.peek_token.as_ref().unwrap().get_literal());
        self.errors.push(msg);
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }


    fn parse_statement(&mut self) -> Option<Statement> {
        if let Some(cur_tok) = self.cur_token.as_ref() {
            match cur_tok {
                Token::Let => {
                    self.parse_let_statement()
                },
                Token::Return => {
                    self.parse_return_statement()
                }
                _ => None
            }
        } else {
            None
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let mut stmt = Statement::new_let(self.cur_token.clone()?);
        if let Statement::Let{
            token: _, 
            ref mut name,
            value: _ 
        } = &mut stmt {

            if !matches!(self.peek_token, Some(Token::Ident(_))){
                return None
            }                 

            self.next_token();

            *name = Some(Identifier::new(self.cur_token.clone()?));
            if !self.expect_peek(Token::Assign){
                return None
            }

            // TODO: Skipping expression until semicolon cuz we noobies
            while !self.cur_token_is(Token::Semicolon) {
                self.next_token();
            }
            return Some(stmt)
        }
        None
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let mut stmt = Statement::new_return(self.cur_token.clone()?);
        if let Statement::Return{
            token: _, 
            value: _ 
        } = &mut stmt {

            if !matches!(self.peek_token, Some(Token::Ident(_))){
                return None
            }                 

            self.next_token();

            *name = Some(Identifier::new(self.cur_token.clone()?));
            if !self.expect_peek(Token::Assign){
                return None
            }

            // TODO: Skipping expression until semicolon cuz we noobies
            while !self.cur_token_is(Token::Semicolon) {
                self.next_token();
            }
            return Some(stmt)
        }
        None
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
        if self.peek_token_is(t.clone()){
            self.next_token();
            return true;
        } 
        self.peek_error(t.clone());
        false
    }
}

#[cfg(test)]
mod test {
    use crate::{ast::{Node, Statement}, lexer::Lexer, token::Token};
    use super::Parser;

    #[test]
    fn test_let_statements(){
        let input = "let x = 5;
let b = 10;
let foobar = 84848;";
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(&p);
        if program.statements.len() != 3 {
            panic!("program.statements does not contain 3 statements");
        }

        let tests = ["x", "y", "foobar"];
        for (i, t) in tests.iter().enumerate() {
            let statement = program.statements.get(i).expect("No statement at index {}");
            if !test_let_statement(statement, t) {
                return;
            }
        }
    }

    #[test]
    fn test_return_statements(){
        let input = "return 5;
return 10;
return 993322;";
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(&p);

        if program.statements.len() != 3 {
            panic!("program.statements does not contain 3 statements");
        }

        let statements = program.statements;
        for stmt in &statements {
            if let Statement::Return { token, value } = stmt {
                if stmt.token_literal() != "return" {
                    panic!("Statement::Return.token is not type return, got {}", stmt.token_literal());
                }
            } else {
                panic!("Statement is not Statement::Return as expected, got {:?}", stmt);
            }
        }
    }
    
    fn check_parser_errors(p: &Parser){
        let errors = &p.errors;

        if errors.is_empty() {
            return
        }

        eprintln!("parser has {} errors", errors.len());
        for msg in errors {
            eprintln!("{msg}");
        }

        panic!();
    }

    fn test_let_statement(statement: &Statement, var_name: &str) -> bool{

        if let Statement::Let { token:_, name, value:_ } = statement {
            let val = &name.as_ref().unwrap().value;
            let token_literal = &name.as_ref().unwrap().token_literal();

            if val != var_name {
                eprintln!("let statement value is not {var_name}, got {}", val);
                return false;
            }

            if token_literal != var_name {
                eprintln!("Nome not {var_name}, got {}", token_literal);
                return false;
            }

            return true
        }
        eprintln!("statement is not type Statement::Let");
        false
    }
}
