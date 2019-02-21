use crate::common::*;
use colored::*;
use rprompt::prompt_reply_stdout;
use std::io::{stdout, Write};
//use std::cell::Cell;

pub struct Prompt {
}

impl Prompt {
    pub fn new() -> Prompt {
        Prompt {
        }
    }

    pub fn proc_with(&self, body: &str, 
                     f: impl FnOnce() -> Result<()>) -> Result<()> {

        print!("{} {} ", "[>]".cyan(), body);
        stdout().flush().unwrap();

        let result = f();
        match result {
            Ok(_) => println!("{}", "[OK]".green()),
            Err(_) => println!("{}", "[ERR]".red()),
        }
        result
    }

    pub fn success(&self, body: &str) {
        let tag = "[SUCCESS]".green();
        println!("{} {}", tag, body)
    }

    pub fn info(&self, body: &str) {
        let tag = "[INFO]".cyan();
        println!("{} {}", tag, body);
    }

    pub fn error(&self, body: &str) {
        let tag = "[ERROR]".red();
        println!("{} {}", tag, body);
    }

    pub fn ask_yesno(&self, question: &str) -> bool {
        let reply = prompt_reply_stdout(
            &format!("{} {} [yN] ", "[?]".cyan(), question)
        ).expect("get prompt reply failed");

        return reply == "y" || reply == "Y";
    }
}
