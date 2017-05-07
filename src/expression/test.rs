use super::*;
use token::Token;

#[test]
fn parsing()
{
    let input = "(label fac (lambda (n)   ; here we pretend that a number sytem exists
        (cond                             ;
            ((num-eq zero n) 'zero)       ; we also pretend that a num-eq function exists, and a zero value
            ('t (num-mul                  ; also the num-mul function
                n                         ;
                (fac                      ;
                    (num-sub n one))))))) ; also a one value, and num-sub";
    
    let expected = Expression::List(List::new(vec![
        Expression::Atom(Atom::new("label")),
        Expression::Atom(Atom::new("fac")),
        Expression::List(List::new(vec![
            Expression::Atom(Atom::new("lambda")),
            Expression::List(List::new(vec![
                Expression::Atom(Atom::new("n"))
            ])),
            Expression::List(List::new(vec![
                Expression::Atom(Atom::new("cond")),
                Expression::List(List::new(vec![
                    Expression::List(List::new(vec![
                        Expression::Atom(Atom::new("num-eq")),
                        Expression::Atom(Atom::new("zero")),
                        Expression::Atom(Atom::new("n"))
                    ])),
                    Expression::List(List::new(vec![
                        Expression::Atom(Atom::new("quote")),
                        Expression::Atom(Atom::new("zero"))
                    ]))
                ])),
                Expression::List(List::new(vec![
                    Expression::List(List::new(vec![
                        Expression::Atom(Atom::new("quote")),
                        Expression::Atom(Atom::new("t"))
                    ])),
                    Expression::List(List::new(vec![
                        Expression::Atom(Atom::new("num-mul")),
                        Expression::Atom(Atom::new("n")),
                        Expression::List(List::new(vec![
                            Expression::Atom(Atom::new("fac")),
                            Expression::List(List::new(vec![
                                Expression::Atom(Atom::new("num-sub")),
                                Expression::Atom(Atom::new("n")),
                                Expression::Atom(Atom::new("one"))
                            ]))
                        ]))
                    ]))
                ]))
            ]))
        ]))
    ]));

    let actual = Expression::parse(Token::lex(input)).unwrap();

    assert_eq!(expected, actual);
}