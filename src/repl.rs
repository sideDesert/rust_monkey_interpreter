use std::io::{stdin, stdout, Write};

use crate::lexer::Lexer;
use crate::parser::Parser;

const PROMPT: &str = ">>";
const MONKEY_FACE: &str = " __,__
                .--. .-\" \"-. .--.
            / .. \\/ .-. .-. \\/ .. 
            | | '| / Y \\ |' | |
            | \\ \\ \\ 0 | 0 / / / |
            \\ '- ,\\.-\"\"\"\"\"\"\"-./, -' /
            ''-' /_ ^ ^ _\\ '-''
                | \\._ _./ |
                \\ \\ '~' / /
                '._ '-=-' _.'
                    '-----";
pub fn start(){

    loop {
        let mut line = String::new();
        print!("{PROMPT} ");
        stdout().flush().unwrap();
        if stdin().read_line(&mut line).unwrap() == 0 {
            return;
        }
        let mut l = Lexer::new(&line);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        if !p.errors().is_empty() {
            print_parse_errors(p.errors());
            continue
        }

        println!("{}", program.statements.iter().map(|stmt| stmt.to_string()).collect::<Vec<String>>().join("\n"));
    }
}

fn print_parse_errors(errors: &[String]){
    println!("{MONKEY_FACE}");
    println!("Whoops! We ran into some monkey business here! :)");
    for msg in errors {
        println!("{msg}");
    }
}
