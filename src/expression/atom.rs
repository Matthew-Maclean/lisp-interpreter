use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Atom(Rc<String>);

impl Atom
{
    pub fn new<S: Into<String>>(s: S) -> Atom
    {
        Atom(Rc::new(s.into()))
    }

    pub fn as_str<'a>(&'a self) -> &'a str
    {
        self.0.as_str()
    }
}

use std::fmt;

impl fmt::Display for Atom
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.as_str())
    }
}