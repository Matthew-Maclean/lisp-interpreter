mod atom;
mod list;

#[cfg(test)]
mod test;

pub use self::atom::Atom;
pub use self::list::List;

use token::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression
{
    Atom(Atom),
    List(List),
}

#[derive(Copy, Clone, Debug)]
pub enum ParseError
{
    TooManyOpenParens,
    TooManyCloseParens,
}

use std::fmt;

impl fmt::Display for ParseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self
        {
            ParseError::TooManyOpenParens => write!(f, "Too many open parenthesis"),
            ParseError::TooManyCloseParens => write!(f, "Too many close parenthesis")
        }
    }
}

impl Expression
{
    pub fn parse(tokens: Vec<Token>) -> Result<Expression, ParseError>
    {
        pub fn parse_inner(tokens: &[Token], start: usize) -> Result<(Expression, usize), ParseError>
        {
            match &tokens[start]
            {
                &Token::Ident(ref name) => return Ok((Expression::Atom(Atom::new(name.as_str())), start + 1)),
                &Token::OpenParen =>
                {
                    let mut index = start + 1;
                    let mut list = Vec::new();

                    loop
                    {
                        if index >= tokens.len()
                        {
                            return Err(ParseError::TooManyOpenParens);
                        }

                        match &tokens[index]
                        {
                            &Token::CloseParen => break,
                            _ =>
                            {
                                let (expr, end) = parse_inner(tokens, index)?;

                                index = end;

                                list.push(expr);
                            }
                        }
                    }

                    return Ok((Expression::List(List::new(list)), index + 1))
                },
                &Token::CloseParen => return Err(ParseError::TooManyCloseParens),
                &Token::Quote =>
                {
                    let (expr, end) = parse_inner(tokens, start + 1)?;

                    return Ok((Expression::List(List::new(vec![
                        Expression::Atom(Atom::new("quote")),
                        expr
                    ])), end))
                }
            }
        }

        Ok(parse_inner(&tokens, 0)?.0)
    }
}

impl fmt::Display for Expression
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            &Expression::Atom(ref atom) => write!(f, "{}", atom),
            &Expression::List(ref list) => write!(f, "{}", list)
        }
    }
}