#![allow(dead_code)]

mod token;
mod expression;
mod eval;

use token::Token;
use expression::Expression;
use eval::eval;

use std::io::{self, BufRead, Write};

fn main()
{
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

        let input = format!("{}\n", stdin
            .lock()
            .lines()
            .next()
            .expect("Could not read next line from stdin")
            .unwrap());
        
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
