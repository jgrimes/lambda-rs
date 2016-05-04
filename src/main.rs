extern crate combine;
use std::io;
use std::io::prelude::*;
use std::collections::LinkedList;
use self::combine::{parser, Parser};
//use self::combine::primitives::{State, Stream, ParseResult};
mod lambda;
use lambda::Lambda;
mod parser;

pub enum ReplResult {
    Quit,
    History,
    Error(String),
    Output(Lambda),
    Str(String)
}

use self::ReplResult::*;

fn print_history(hist: &LinkedList<String>) -> () {
    for (i, item) in hist.iter().enumerate() {
        println!("{}: {}", i, item)
    }
}

fn eval (str: &String) -> ReplResult {
    match str.trim().as_ref() {
        "quit"    => Quit,
        "history" => History,
        s         => {
            match parser(parser::lambda).parse(s.clone()) {
                Ok(o) => Output(o.0.nf()),
                Err(e) => Error(e.to_string())
            }

        }
    }
}

fn main() {
    let mut history = LinkedList::new();
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match eval(&input) {
                    Quit => {
                        println!("Bye");
                        break
                    },
                    History => {
                        print_history(&history)
                    },
                    Str(o) => {
                        println!("Result: {}", o);
                        history.push_back(o)
                    },
                    Output(o) => {
                        println!("Result: {:?}", o)
                    }
                    Error(e) => println!("error: {}", e)
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
