#![allow(dead_code)]

use crate::ast::{Expression, Identifier, InfixExpression, IntegerLiteral, PrefixExpression, Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;
use crate::{BlockStatement, Boolean, CallExpression, FunctionLiteral, IfExpression};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest = 0,
    Equals = 1,
    LessGreater = 2,
    Sum = 3,
    Product = 4,
    Prefix = 5,
    Call = 6,
}

impl Precedence {
    pub fn as_int(&self) -> i32 {
        *self as i32
    }

    pub fn token(token: Token) -> Self {
        match token {
            Token::Eq => Self::Equals,
            Token::NotEq => Self::Equals,
            Token::Lt => Self::LessGreater,
            Token::Gt => Self::LessGreater,
            Token::Plus => Self::Sum,
            Token::Minus => Self::Sum,
            Token::Slash => Self::Product,
            Token::Asterisk => Self::Product,
            Token::Lparen => Self::Call,
            _ => Self::Lowest
        }
    }
}

pub struct Parser<'parser> {
    pub lexer: &'parser mut Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
}

use Precedence::*;

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

    pub fn prefix_parse_fns(& mut self, token: &Token) -> Option<Box<dyn Expression>> {
        let cur_token = self.cur_token.clone()?;

        match token {
            Token::Ident(_) => Some(Box::new(Identifier {
                value: cur_token.get_literal(),
                token: cur_token.clone(),
            })),
            Token::Int(_) => self.parse_integer_literal(),
            Token::Bang => self.parse_prefix_expression(),
            Token::Minus => self.parse_prefix_expression(),
            Token::True => self.parse_boolean(),
            Token::False => self.parse_boolean(),
            Token::If => self.parse_if_expression(),
            Token::Function => self.parse_function_literal(),
            _ => None,
        }
    }


    pub fn infix_parse_fns(&mut self,token: &Token, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        match token {
            Token::Plus => self.parse_infix_expression(left),
            Token::Minus => self.parse_infix_expression(left),
            Token::Slash => self.parse_infix_expression(left),
            Token::Asterisk => self.parse_infix_expression(left),
            Token::Eq => self.parse_infix_expression(left),
            Token::NotEq => self.parse_infix_expression(left),
            Token::Lt => self.parse_infix_expression(left),
            Token::Gt => self.parse_infix_expression(left),
            Token::Lparen => self.parse_call_expression(left).map(|expr| expr as Box<dyn Expression>),
            _ => None,
        }
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
        let msg = format!(
            "expected next token to be {}, got {:?} instead",
            &t.get_literal(),
            self.peek_token.as_ref().unwrap().get_literal()
        );
        self.errors.push(msg);
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        if let Some(cur_tok) = self.cur_token.as_ref() {
            match cur_tok {
                Token::Let => self.parse_let_statement(),
                Token::Return => self.parse_return_statement(),
                _ => self.parse_expression_statement(),
            }
        } else {
            None
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !matches!(self.peek_token, Some(Token::Ident(_))) {
            return None;
        }

        self.next_token();

        let name = Some(Identifier::new(self.cur_token.clone()?));
        if !self.expect_peek(Token::Assign) {
            return None;
        }

        // TODO: Skipping expression until semicolon cuz we noobies
        self.next_token();

        let value =  self.parse_expression(Lowest);
        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Let { token: Token::Let, name,  value })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {

        self.next_token();
        let value = self.parse_expression(Lowest);

        if self.peek_token_is(Token::Semicolon){
            self.next_token();
        }

        Some(Statement::Return { token: Token::Return,  value })
    }

    fn no_prefix_parse_fn_errors(&mut self, t: Token) {
        let msg = format!("no prefix parse function for {} found", t.get_literal());
        self.errors.push(msg);
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        let t = self.cur_token.clone();
        let mut left_exp;

        if let Some(token) = t {
            match self.prefix_parse_fns(&token) {
                Some(exp) => left_exp = Some(exp),
                None => {
                    self.no_prefix_parse_fn_errors(self.cur_token.clone()?); 
                    return None
                }
            };
        } else {
            eprintln!("Token is of type None");
            return None
        }

        while !self.peek_token_is(Token::Semicolon) && precedence < self.peek_precedence() {
            let pt = self.peek_token.clone()?;
            let prev_left = left_exp?;
            self.next_token();
            left_exp = self.infix_parse_fns(&pt, prev_left);
        }

        left_exp
    }

    fn parse_integer_literal(&self) -> Option<Box<dyn Expression>> {
        let cur_token = self.cur_token.clone();
        if let Some(token) = cur_token {
            let parsed_token = token.get_literal().parse::<i32>();
            return match parsed_token {
                Ok(val) => Some(Box::new(IntegerLiteral::new(token, val))),
                Err(e) => {
                    eprintln!("This token cannot pe parsed as integer");
                    eprintln!("{}", e);
                    None
                }
            };
        }

        None
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<dyn Expression>> {
        let token = self.cur_token.clone()?;
        self.next_token();

        Some(Box::new(PrefixExpression {
            token: token.clone(),
            operator: token.get_literal(),
            right: self.parse_expression(Precedence::Prefix)?,
        }))
    }

    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        let precedence = self.cur_precedence();
        let cur_token = self.cur_token.clone()?;
        self.next_token();

        Some(Box::new(InfixExpression{
            token: cur_token.clone(),
            operator: cur_token.get_literal(),
            right: self.parse_expression(precedence)?,
            left
        }))
    }

    fn parse_boolean(&self) -> Option<Box<dyn Expression>> {
        Some(Box::new(
            Boolean{
                token: self.cur_token.clone()?,
                value: self.cur_token_is(Token::True)
            }
        ))
    }

    fn parse_if_expression(&mut self) -> Option<Box<dyn Expression>> {
        if !self.expect_peek(Token::Lparen){
            return None
        }

        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(Token::Rparen){
            return None
        }
        if !self.expect_peek(Token::Lbrace){
            return None
        }

        let consequence = self.parse_block_statement();
        let mut alternative = None;
        if self.peek_token_is(Token::Else) {
            self.next_token();

            if !self.expect_peek(Token::Lbrace){
                return None
            }
            alternative = Some(self.parse_block_statement());
        }

        Some(Box::new(IfExpression{
            token: self.cur_token.clone()?,
            alternative,
            condition,
            consequence,
        }))
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let token = self.cur_token.clone().unwrap();
        let mut statements: Vec<Statement> = vec![];
        self.next_token();

        while !self.cur_token_is(Token::Rbrace) && !self.cur_token_is(Token::Eof) {
            let stmt = self.parse_statement();
            if let Some(stmt) = stmt {
                statements.push(stmt);
            }
            self.next_token();
        }

        BlockStatement{
            token,
            statements
        }
    }

    fn parse_function_literal(&mut self) -> Option<Box<dyn Expression>> {
        if !self.expect_peek(Token::Lparen) {
            return None
        }

        let parameters = self.parse_function_parameters()?;
        if !self.expect_peek(Token::Lbrace) {
            return None
        }

        let body = self.parse_block_statement();

        Some(Box::new(FunctionLiteral{
            token: self.cur_token.clone().unwrap(),
            parameters,
            body
        }))
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Identifier>>{
        let mut identifier = vec![];

        if self.peek_token_is(Token::Rparen) {
            self.next_token();
            return Some(identifier)
        }

        self.next_token();
        let cur_token = self.cur_token.clone().unwrap();
        let ident = Identifier{
            value: cur_token.get_literal(),
            token: cur_token
        };

        identifier.push(ident);

        while self.peek_token_is(Token::Comma) {
            self.next_token();
            self.next_token();

            let ident = Identifier{
                token: self.cur_token.clone().unwrap(),
                value: self.cur_token.clone().unwrap().get_literal()
            };

            identifier.push(ident);
        }

        if !self.expect_peek(Token::Rparen){
            return None
        }

        Some(identifier)
    }

    fn parse_call_expression(&mut self, function: Box<dyn Expression>) -> Option<Box<CallExpression>> {
        let arguments = self.parse_call_arguments();
        Some(Box::new(CallExpression{
            token: self.cur_token.clone()?,
            arguments: arguments?,
            function: Some(function)
        }))
    } 

    fn parse_call_arguments(&mut self) -> Option<Vec<Box<dyn Expression>>> {
        let mut args = vec![];
        if self.peek_token_is(Token::Rparen) {
            self.next_token();
            return Some(args);
        }

        self.next_token();
        let exp = self.parse_expression(Lowest).expect("expression is None");

        args.push(exp);

        while self.peek_token_is(Token::Comma){
            self.next_token();
            self.next_token();
            if let Some(exp) = self.parse_expression(Lowest){
                args.push(exp);
            }
        }

        if !self.expect_peek(Token::Rparen){
            return None;
        } 

        Some(args)
    }

    fn parse_grouped_expression(&mut self) -> Option<Box<dyn Expression>> {
        self.next_token();

        let exp = self.parse_expression(Precedence::Lowest);

        if !self.expect_peek(Token::Rparen) {
            return None
        }

        exp
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let stmt = Statement::Expression {
            token: self.cur_token.clone()?,
            expression: self.parse_expression(Lowest),
        };

        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }

        Some(stmt)
    }

    fn cur_token_is(&self, t: Token) -> bool {
        match self.cur_token.as_ref() {
            None => false,
            Some(tok) => *tok == t,
        }
    }

    fn peek_token_is(&self, t: Token) -> bool {
        match self.peek_token.as_ref() {
            None => false,
            Some(tok) => *tok == t,
        }
    }

    fn peek_precedence(&self) -> Precedence {
        if let Some(pt) = self.peek_token.clone() {
            return Precedence::token(pt)
        }
        Precedence::Lowest
    }

    fn cur_precedence(&self) -> Precedence {
        if let Some(ct) = self.cur_token.clone() {
            return Precedence::token(ct)
        }
        Precedence::Lowest
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        if self.peek_token_is(t.clone()) {
            self.next_token();
            return true;
        }
        self.peek_error(t.clone());
        false
    }
}

#[cfg(test)]
mod test {

    use std::{any::Any, ops::Deref};
    use super::Parser;
    use crate::{FunctionLiteral, Lexer};
    use crate::ast::{Boolean, Expression, Identifier, InfixExpression, IntegerLiteral, Node, PrefixExpression, Statement};

    #[test]
    fn test_let_statements() {
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
    fn test_return_statements() {
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
            if let Statement::Return { token: _, value: _ } = stmt {
                if stmt.token_literal() != "return" {
                    panic!(
                        "Statement::Return.token is not type return, got {}",
                        stmt.token_literal()
                    );
                }
            } else {
                panic!(
                    "Statement is not Statement::Return as expected, got {}",
                    stmt
                );
            }
        }
    }

    fn check_parser_errors(p: &Parser) {
        let errors = &p.errors;

        if errors.is_empty() {
            return;
        }

        eprintln!("parser has {} errors", errors.len());
        for msg in errors {
            eprintln!("{msg}");
        }

        panic!();
    }

    fn test_let_statement(statement: &Statement, var_name: &str) -> bool {
        if let Statement::Let {
            token: _,
            name,
            value: _,
        } = statement {
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

            return true;
        }
        eprintln!("statement is not type Statement::Let");
        false
    }

    // Need to complete this
    #[test]
    fn test_identifier_expression() {
        let input = "foobar";
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(1, program.statements.len() as i32);
        if let Some(stmt) = program.statements.first() {
            match stmt {
                Statement::Expression {
                    token: _,
                    expression,
                } => {
                    if let Some(exp) = expression{
                        self::test_literal_expression(exp.deref(), &String::from("foobar"));
                    }
                }
                _ => panic!("Statement received is not type Statement::Expression"),
            }
        }
    }

    #[test]
    fn test_boolean_expression() {
        let input = "true";
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(1, program.statements.len() as i32);
        if let Some(stmt) = program.statements.first() {
            match stmt {
                Statement::Expression {
                    token: _,
                    expression,
                } => {
                    match expression {
                        Some(exp) => {
                            self::test_boolean_literal(exp.deref(), true);
                        },
                        None => panic!("expression is None")
                    }
                }
                _ => panic!("Statement received is not type Statement::Expression"),
            }
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "238784;";
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(&p);
        assert_eq!(1, program.statements.len() as i32);

        if let Some(stmt) = program.statements.first() {
            match stmt {
                Statement::Expression {
                    token: _,
                    expression,
                } => {
                    if let Some(exp) = expression {
                        self::test_literal_expression(exp.deref(), &238784);
                    }
                }
                _ => panic!("Statement received is not type Statement::Expression"),
            }
        }
    }

    #[test]
    fn test_parsing_prefix_expression() {
        struct Input<'input> {
            input: &'static str,
            operator: &'static str,
            value: &'input dyn Any,
        }

        let input = [
            Input {
                input: "!5;",
                operator: "!",
                value: &5,
            },
            Input {
                input: "-15",
                operator: "-",
                value: &15,
            },
            Input {
                input: "!true",
                operator: "!",
                value: &true,
            },
            Input {
                input: "!false",
                operator: "!",
                value: &false,
            },
        ];

        for test_case in input {
            let mut l = Lexer::new(test_case.input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            check_parser_errors(&p);

            if program.statements.len() != 1 {
                eprintln!("{:?}", program.statements);
                panic!("program.statements.len() != 1");
            }

            if let Some(stmt) = program.statements.first() {
                match stmt {
                    Statement::Expression {
                        token: _,
                        expression,
                    } => match expression {
                        Some(expression) => {
                            match expression
                                .as_ref()
                                .as_any()
                                .downcast_ref::<PrefixExpression>()
                            {
                                Some(exp) => {
                                    assert_eq!(exp.operator, test_case.operator);
                                    self::test_literal_expression(exp.right.deref(), &test_case.value);
                                    if !test_literal_expression(
                                        exp.right.as_ref(),
                                        test_case.value,
                                    ) {
                                        return;
                                    }
                                }
                                None => panic!("Expression is not of type PrefixExpression"),
                            }
                        }
                        None => panic!("Expression is None"),
                    },
                    _ => panic!("Statement is not of type Statement::Expression"),
                }
            }
        }
    }

    fn test_integer_literal(il: &dyn Expression, value: i32) -> bool {
        match il.as_any().downcast_ref::<IntegerLiteral>() {
            Some(integ) => {
                assert_eq!(integ.value, value);
                assert_eq!(integ.token_literal(), format!("{}", value));
            }
            None => panic!("il is not an IntegerLiteral"),
        }
        true
    }

    #[test]
    fn test_parsing_infix_expression() {
        struct Input<'input> {
            input: &'static str,
            left_value: &'input dyn Any,
            operator: &'static str,
            right_value: &'input dyn Any,
        }

        impl<'input> Input<'input> {
            fn new(
                input: &'static str,
                left_value: &'input dyn Any,
                operator: &'static str,
                right_value: &'input dyn Any,
            ) -> Self {
                Self {
                    input,
                    left_value,
                    operator,
                    right_value,
                }
            }
        }

        let input = [
            Input::new("5 + 5", &5, "+", &5),
            Input::new("5/ 5", &5, "/", &5),
            Input::new("5-5", &5, "-", &5),
            Input::new("5*5", &5, "*", &5),
            Input::new("5>5", &5, ">", &5),
            Input::new("5<5", &5, "<", &5),
            Input::new("5 == 5", &5, "==", &5),
            Input::new("5 != 5", &5, "!=", &5),
            Input::new("true == true", &true, "==", &true),
            Input::new("true != false", &true, "!=", &false),
            Input::new("false == false", &false, "==", &false),
        ];

        for test_case in input {
            let mut l = Lexer::new(test_case.input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            check_parser_errors(&p);

            if program.statements.len() != 1 {
                eprintln!("{:?}", program.statements);
                panic!("program.statements.len() != 1");
            }

            if let Some(stmt) = program.statements.first() {
                match stmt {
                    Statement::Expression {
                        token: _,
                        expression,
                    } => match expression {
                        Some(expression) => {
                            if !test_infix_expression(expression.deref(), test_case.left_value, test_case.operator, test_case.right_value){
                                return
                            }
                        }
                        None => panic!("Expression is None"),
                    },
                    _ => panic!("Statement is not of type Statement::Expression"),
                }
            }
        }
    }

    fn test_identifier(exp: &dyn Expression, value: &str) -> bool {
        match exp.as_any().downcast_ref::<Identifier>() {
            Some(ident) => {
                assert_eq!(ident.value, value, "exp.value is not {}. got {}.", value, ident.value);
                assert_eq!(ident.token_literal(), value, "ident.token_literal() is not {}. got {}", value, ident.value);
            }
            None => panic!("exp is not an Identifier")
        }
        true
    }

    fn test_literal_expression(exp: &dyn Expression, expected: &dyn Any) -> bool {
        if let Some(v) = expected.downcast_ref::<i32>() {
            return test_integer_literal( exp, *v);
        } else if let Some(v) = expected.downcast_ref::<i64>() {
            return test_integer_literal( exp, *v as i32);
        } else if let Some(v) = expected.downcast_ref::<String>() {
            return test_identifier(exp, v);
        } else if let Some(v) = expected.downcast_ref::<bool>() {
            return test_boolean_literal(exp, *v);
        }
        false
    }

    fn test_boolean_literal(exp: &dyn Expression, value: bool) -> bool {
        match exp.as_any().downcast_ref::<Boolean>() {
            Some(bo) => {
                assert_eq!(bo.value, value, "bo.value is not {}. got {}", value, bo.value);
                assert_eq!(bo.token_literal(), format!("{}", value), "bo.token_literal() is not {}. got {}", value, bo.token_literal())
            },
            None => panic!("bo is not of type Boolean")
        }

        true
    }

    fn test_infix_expression(exp: &dyn Expression, left: &dyn Any, operator: &str, right: &dyn Any) -> bool {
        match exp.as_any().downcast_ref::<InfixExpression>() {
            Some(op_exp) => {
                if !test_literal_expression(op_exp.left.deref(), left){
                    let left_str = left.downcast_ref::<&str>().expect("Can't downcast left to String");
                    let error_msg= format!("op_exp.left is not {}. got {}", left_str, op_exp.left);
                    panic!("{}", error_msg)
                }

                if op_exp.operator != operator {
                    let error_msg= format!("op_exp.operator is not {}. got {}", operator, op_exp.operator);
                    panic!("{}", error_msg);
                }

                if !test_literal_expression(op_exp.right.deref(), right){
                    let right_str = right.downcast_ref::<String>().expect("Can't downcast right to String");
                    let error_msg= format!("op_exp.righ is not {}. got {}", right_str, op_exp.right);
                    panic!("{}", error_msg);
                }
            }
            None => panic!("exp is not an InfixExpression")
        }
        true
    }

    #[test]
    fn test_operator_precedence_parsing(){
        struct Input {
            input: &'static str,
            expected: &'static str
        }

        impl Input {
            fn new(input: &'static str, expected: &'static str) -> Self{
                Self{
                    input,
                    expected
                }
            }
        }

        let tests = [
            Input::new("true", "true"),
            Input::new("false", "false"),
            Input::new("3 > 5 == false", "((3 > 5) == false)"),
            Input::new("3 < 5 == true", "((3 < 5) == true)"),
            Input::new("a + add(b * c) + d", "((a + add((b * c), )) + d)"),
            Input::new("add(a + b + c * d / f + g)","add((((a + b) + ((c * d) / f)) + g), )"),
            Input::new("add(a, b, add(b, c))","add(a, b, add(b, c, ), )"),
            //Input::new("add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))","add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8), ), )"),
        ];

        for tc in tests {
            let mut l = Lexer::new(tc.input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();

            println!("{:?}", program.statements);
            assert_eq!(program.statements.len(), 1, "program.statements.len() is not 1. got {}.\n\n DUMP:\n{:?}", program.statements.len(), program.statements);
            if let Some(stmt) = program.statements.first() {
                match stmt {
                    Statement::Expression { token: _, expression } => {
                        if let Some(exp) = expression {
                            assert_eq!(format!("{}", exp), format!("{}", tc.expected), "exp is not {}. got {}", tc.expected, exp);
                        } else {
                            panic!("expression is none")
                        }
                    },
                    _ => panic!("stmt is not of type Statement::Expression")
                }
            }
        }
    }

    #[test]
    fn test_functional_literal_parsing(){
        let input = "fn(x,y){ x+y; }";
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(program.statements.len(), 1, "program.statements is not 1. got {}", program.statements.len());

        match program.statements.first().unwrap() {
            Statement::Expression { token: _, expression } => {
                match expression {
                    Some(exp) => {
                        match exp.as_any().downcast_ref::<FunctionLiteral>() {
                            Some(fn_ltrl) => {
                                assert_eq!(fn_ltrl.parameters.len(), 2, "fn_ltrl.parameters is not 2. got {}", fn_ltrl.parameters.len());
                                test_literal_expression(&fn_ltrl.parameters[0], &String::from("x"));
                                test_literal_expression(&fn_ltrl.parameters[1], &String::from("y"));

                                assert_eq!(fn_ltrl.body.statements.len(), 1, "fn_ltrl.body.statements is not 1. got {}", fn_ltrl.body.statements.len());

                                match &fn_ltrl.body.statements[0] {
                                    Statement::Expression { token: _, expression } => {
                                        test_infix_expression(expression.as_ref().unwrap().deref(), &String::from("x"), "+", &String::from("y"));
                                    },
                                    _ => panic!("function body statement is not Expression")
                                }
                            },
                            None => panic!("expression cannot be casted to a FunctionLiteral")
                        }
                    },
                    None => panic!("expression is None")
                }
            },
            _ => panic!("Statement is not of type Expression")
        }
    }

    #[test]
    fn test_function_parameter_parsing(){
        struct Input {
            input: &'static str,
            expected: Vec<&'static str>
        }

        impl Input {
            fn new(input: &'static str, expected: Vec<&'static str>) -> Self {
                Input {
                    input,
                    expected
                }
            }

        }

        let tests=  [
            Input::new("fn(){}", [].to_vec()),
            Input::new("fn(x){}", ["x"].to_vec()),
            Input::new("fn(x,y,z){}", ["x","y","z"].to_vec())
        ];

        for test in tests {
            let input = test.input;

            let mut l = Lexer::new(input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            check_parser_errors(&p);

            assert_eq!(program.statements.len(), 1, "program.statements is not 1. got {}", program.statements.len());

            match program.statements.first().unwrap() {
                Statement::Expression { token: _, expression } => {
                    match expression {
                        Some(exp) => {
                            match exp.as_any().downcast_ref::<FunctionLiteral>() {
                                Some(fn_ltrl) => {
                                    assert_eq!(fn_ltrl.parameters.len(), test.expected.len(), "fn_ltrl.parameters is not {}. got {}", test.expected.len(), fn_ltrl.parameters.len());


                                    for (i, ident) in test.expected.iter().enumerate() {
                                        if !test.expected.is_empty(){
                                            println!("{:?}", fn_ltrl.parameters);
                                            println!("{:?}", &String::from(*ident));

                                            test_literal_expression(&fn_ltrl.parameters[i], &String::from(*ident));
                                        }
                                    }
                                },
                                None => panic!("expression cannot be casted to a FunctionLiteral")
                            }
                        },
                        None => panic!("expression is None")
                    }
                },
                _ => panic!("Statement is not of type Expression")
            }
        }
    }
}
