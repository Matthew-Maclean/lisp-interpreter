mod atom;
mod list;

pub use self::atom::Atom;
pub use self::list::List;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression
{
    Atom(Atom),
    List(List),
}

use std::fmt;

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