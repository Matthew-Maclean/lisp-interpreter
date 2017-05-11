#![allow(dead_code)]

extern crate clap;

mod token;
mod expression;
mod eval;

use token::Token;
use expression::Expression;
use eval::eval;

fn main()
{
    use clap::*;

    let matches = App::new("lisp-interpreter")
        .author("Matthew Maclean")
        .about("Minimal lisp interpreter")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .help("interprets from a file")
            .takes_value(true)
            .value_name("FILE")
            .required(false))
        .get_matches();
    
    if matches.is_present("file")
    {
        file(matches.value_of("file")
            .expect("the --file option requires an input file"));
    }
    else
    {
        repl();
    }
}

fn file(path: &str)
{
    use std::fs::File;
    use std::io::Read;

    let mut file = match File::open(path)
    {
        Ok(f) => f,
        Err(_) => bad_exit("Unable to open file")
    };

    let input =
    {
        let mut s = String::new();
        match file.read_to_string(&mut s)
        {
            Ok(_) => {},
            Err(_) => bad_exit("Unable to read from file")
        }
        s
    };

    let tokens = Token::lex(&input);

    match Expression::parse(tokens)
    {
        Ok(expr) => match eval(expr)
        {
            Ok(val) => println!("{}", val),
            Err(err) => println!("err: {}", err)
        },
        Err(err) => println!("err: {}", err)
    }
}

fn repl()
{
    use std::io::{self, Write};

    println!("comment 'exit' to exit");

    let mut open = 0;
    let mut tokens = Vec::new();

    let stdin = io::stdin();

    loop
    {
        if open == 0
        {
            print!(">>> : ");
        }
        else if open > 999
        {
            print!("99+ : ");
        }
        else
        {
            print!("{:>03} : ", open);
        }

        io::stdout().flush().expect("Could not flush stdout");

        let input = format!("{}\n", readln(&stdin));
        
        if input == "; exit\n"
        {
            std::process::exit(0);
        }

        tokens.append(&mut Token::lex(&input));

        open = match Token::count_parens(&tokens)
        {
            Some(o) => o,
            None =>
            {
                println!("err : Too many close close parenthesis");
                tokens.clear();
                0
            }
        };

        if open == 0 && tokens.len() != 0
        {
            match Expression::parse(tokens.clone())
            {
                Ok(expr) => match eval(expr)
                {
                    Ok(res) => println!("<<< : {}", res),
                    Err(err) => println!("err : {}", err)
                },
                Err(err) => println!("err : {}", err)
            }

            tokens.clear();
        }
    }
}

fn readln(stdin: &::std::io::Stdin) -> String
{
    use std::io::BufRead;

    stdin.lock()
        .lines()
        .next()
        .expect("Cound not read line from stdin")
        .unwrap()
}

fn bad_exit(msg: &str) -> !
{
    println!("{}", msg);
    ::std::process::exit(1);
}