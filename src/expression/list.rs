use super::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct List(Vec<Expression>);

impl List
{
    pub fn new(v: Vec<Expression>) -> List
    {
        List(v)
    }

    pub fn as_slice<'a>(&'a self) -> &'a [Expression]
    {
        &self.0[0..self.0.len()]
    }

    pub fn len(&self) -> usize
    {
        self.0.len()
    }
}

use std::fmt;

impl fmt::Display for List
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self.len()
        {
            0 => write!(f, "()"),
            1 => write!(f, "({})", self.as_slice()[0]),
            _ =>
            {
                let mut temp = String::new();

                for item in self.as_slice()[0..self.len() - 1].iter()
                {
                    temp.push_str(&format!("{} ", item));
                }

                temp.push_str(&format!("{}", self.as_slice()[self.len() - 1]));

                write!(f, "({})", temp)
            }
        }
    }
}