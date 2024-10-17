mod lexer;
mod token;
mod repl;
mod ast;
mod parser;

use crate::ast::*;
use crate::parser::*;
use crate::lexer::*;

mod welcome {
    use users::{get_current_uid, get_user_by_uid};
    pub fn print(){
        let welcome_msg = "Hey there! This is the monkey programming language";
        if let Some(user) = get_user_by_uid(get_current_uid()) {
            if let Some(name) = user.name().to_str() {
                println!("Hey {name}! This is the monkey programming language");
            } else {
                println!("{welcome_msg}");
            }
        }
        println!("Feel free to type in commands!");

    }
}



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
        Input::new("add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))","add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8), ), )"),
    ];

    for tc in tests {
        let mut l = Lexer::new(tc.input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();

        println!("{:?}", program.statements);
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
fn test_parsing_infix_expression() {
    struct Input {
        input: &'static str,
        left_value: i32,
        operator: &'static str,
        right_value: i32,
    }

    impl Input {
        fn new(
            input: &'static str,
            left_value: i32,
            operator: &'static str,
            right_value: i32,
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
        Input::new("5+5", 5, "+", 5),
        Input::new("5/ 5", 5, "/", 5),
        Input::new("5-5", 5, "-", 5),
        Input::new("5*5", 5, "*", 5),
        Input::new("5>5", 5, ">", 5),
        Input::new("5<5", 5, "<", 5),
        Input::new("5 == 5", 5, "==", 5),
        Input::new("5 != 5", 5, "!=", 5),
    ];

    for test_case in input {
        let mut l = Lexer::new(test_case.input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();

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
                            .downcast_ref::<InfixExpression>()
                        {
                            Some(exp) => {
                                assert_eq!(exp.operator, test_case.operator);
                                if !test_integer_literal(
                                    exp.right.as_ref(),
                                    test_case.left_value,
                                ) {
                                    return;
                                }

                                if !test_integer_literal(
                                    exp.left.as_ref(),
                                    test_case.right_value,
                                ) {
                                    return;
                                }
                            }
                            None => panic!("Expression is not of type InfixExpression"),
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

fn main() {
    test_operator_precedence_parsing();
    welcome::print();
    repl::start();
}
