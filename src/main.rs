mod lexer;
mod token;
mod repl;
mod ast;
mod parser;

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
fn main() {
    welcome::print();
    repl::start();
}
