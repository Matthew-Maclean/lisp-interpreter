use std::rc::Rc;
use std::collections::HashMap;

use expression::*;

pub fn eval(input: Expression) -> Result<Expression, String>
{
    fn eval_inner(input: Expression, stack: &[Rc<HashMap<String, Expression>>]) -> Result<Expression, String>
    {
        match input
        {
            Expression::Atom(atom) => match stack_lookup(&stack, atom.as_str())
            {
                Some(value) => Ok(value),
                None => Err(format!("Could not find substitution for atom '{}'", atom))
            },
            Expression::List(list) => match list.len()
            {
                0 => Err(format!("The empty list has no meaning")),
                _ => match &list.as_slice()[0]
                {
                    &Expression::Atom(ref first) => match first.as_str()
                    {
                        "quote" => match list.len()
                        {
                            2 => Ok(list.as_slice()[1].clone()),
                            n => Err(format!("quote expects one argument, not {}", n - 1))
                        },
                        "atom" => match list.len()
                        {
                            2 => match eval_inner(list.as_slice()[1].clone(), stack)?
                            {
                                Expression::Atom(_) => Ok(Expression::Atom(Atom::new("t"))),
                                _ => Ok(Expression::List(List::new(vec![])))
                            },
                            n => Err(format!("atom expects one argument, not {}", n - 1))
                        },
                        "eq" => match list.len()
                        {
                            3 => match (eval_inner(list.as_slice()[1].clone(), stack)?, eval_inner(list.as_slice()[2].clone(), stack)?)
                            {
                                (Expression::Atom(ref left), Expression::Atom(ref right)) if left.as_str() == right.as_str() => Ok(Expression::Atom(Atom::new("t"))),
                                (Expression::List(ref left), Expression::List(ref right)) if left.len() == 0 && right.len() == 0 => Ok(Expression::Atom(Atom::new("t"))),
                                _ => Ok(Expression::List(List::new(vec![])))
                            },
                            n => Err(format!("eq expects two arguments, not {}", n - 1))
                        },
                        "car" => match list.len()
                        {
                            2 => match eval_inner(list.as_slice()[1].clone(), stack)?
                            {
                                Expression::List(arg) => match arg.len()
                                {
                                    0 => Err(format!("car expects a non-empty list as an argument")),
                                    _ => Ok(arg.as_slice()[0].clone())
                                },
                                _ => Err(format!("car expects a list as an argument"))
                            },
                            n => Err(format!("car expects one argument, not {}", n - 1))
                        },
                        "cdr" => match list.len()
                        {
                            2 => match eval_inner(list.as_slice()[1].clone(), stack)?
                            {
                                Expression::List(arg) => match arg.len()
                                {
                                    0 => Err(format!("cdr expects a non-empty list as an argument")),
                                    _ => Ok(Expression::List(List::new(arg.as_slice()[1..arg.len()].to_vec())))
                                },
                                _ => Err(format!("cdr expects a list as an argument"))
                            },
                            n => Err(format!("cdr expects one argument, not {}", n - 1))
                        },
                        "cons" => match list.len()
                        {
                            3 => match (eval_inner(list.as_slice()[1].clone(), stack)?, eval_inner(list.as_slice()[2].clone(), stack)?)
                            {
                                (Expression::Atom(first), Expression::List(rest)) =>
                                {
                                    let mut tmp = vec![Expression::Atom(first)];
                                    for item in rest.as_slice().iter()
                                    {
                                        tmp.push(item.clone());
                                    }
                                    Ok(Expression::List(List::new(tmp)))
                                },
                                _ => Err(format!("cons expects an atom and a list as arguments"))
                            },
                            n => Err(format!("cons expects two arguments, not {}", n - 1))
                        },
                        "cond" => match list.len()
                        {
                            1 => Err(format!("cond expects at least one argument")),
                            _ =>
                            {
                                for item in list.as_slice()[1..list.len()].iter()
                                {
                                    match item
                                    {
                                        &Expression::List(ref pair) =>
                                        {
                                            if pair.len() != 2
                                            {
                                                return Err(format!("each pair in a cond argument list should have two elements, not {}", pair.len()));
                                            }

                                            match eval_inner(pair.as_slice()[0].clone(), stack)?
                                            {
                                                Expression::Atom(_) => return eval_inner(pair.as_slice()[1].clone(), stack),
                                                _ => {}
                                            }
                                        },
                                        _ => return Err(format!("cond expects pairs as arguments"))
                                    }
                                }

                                Err(format!("no first element in any pair in cond was evaluated as true"))
                            }
                        },
                        name => match stack_lookup(&stack, name)
                        {
                            Some(Expression::List(func)) => call_expression(stack, func, &list.as_slice()[1..list.len()]),
                            Some(Expression::Atom(func)) => Err(format!("Cannot call atom '{}' as if it were a function", func)),
                            None => Err(format!("Could not find substitution for atom '{}'", name))
                        }
                    },
                    &Expression::List(ref first) => call_expression(stack, first.clone(), &list.as_slice()[1..list.len()])
                }
            }
        }
    }

    fn call_expression(stack: &[Rc<HashMap<String, Expression>>], func: List, arguments: &[Expression]) -> Result<Expression, String>
    {
        match func.len()
        {
            0 => Err(format!("Cannot call the empty list as a function")),
            3 => match &func.as_slice()[0]
            {
                &Expression::Atom(ref atom) => match atom.as_str()
                {
                    "lambda" => match &func.as_slice()[1]
                    {
                        &Expression::List(ref args) =>
                        {
                            if args.len() != arguments.len()
                            {
                                return Err(format!("Tried to call a function that accepts {} arguments with {} arguments", args.len(), arguments.len()))
                            }

                            if !args.as_slice().iter().all(|e| match e
                            {
                                &Expression::Atom(_) => true,
                                _ => false
                            })
                            {
                                return Err(format!("All elements in a function's argument list must be atoms"))
                            }

                            let argument_values =
                            {
                                let mut v = Vec::new();
                                
                                for arg in arguments.iter()
                                {
                                    v.push(eval_inner(arg.clone(), stack)?)
                                }

                                v
                            };

                            let subs = args.as_slice().iter()
                                .map(|e| match e
                                {
                                    &Expression::Atom(ref atom) => atom.as_str().to_owned(),
                                    _ => unreachable!()
                                })
                                .zip(argument_values.into_iter())
                                .collect::<HashMap<_, _>>();
                            
                            eval_inner(func.as_slice()[2].clone(), &stack_push(stack, subs))
                        },
                        _ => Err(format!("The argument list of a function must be a list"))
                    },
                    "macro" => match &func.as_slice()[1]
                    {
                        &Expression::List(ref args) =>
                        {
                            if args.len() != arguments.len()
                            {
                                return Err(format!("Tried to call a macro that accepts {} arguments with {} arguments", args.len(), arguments.len()))
                            }

                            if !args.as_slice().iter().all(|e| match e
                            {
                                &Expression::Atom(_) => true,
                                _ => false
                            })
                            {
                                return Err(format!("All elements in a macros's argument list must be atoms"))
                            }

                            let argument_values =
                            {
                                let mut v = Vec::new();
                                
                                for arg in arguments.iter()
                                {
                                    v.push(arg.clone());
                                }

                                v
                            };

                            let subs = args.as_slice().iter()
                                .map(|e| match e
                                {
                                    &Expression::Atom(ref atom) => atom.as_str().to_owned(),
                                    _ => unreachable!()
                                })
                                .zip(argument_values.into_iter())
                                .collect::<HashMap<_, _>>();
                            
                            eval_inner(func.as_slice()[2].clone(), &stack_push(stack, subs))
                        },
                        _ => Err(format!("The argument list of a macro must be a list"))
                    },
                    "label" => match &func.as_slice()[1]
                    {
                        &Expression::Atom(ref name) =>
                        {
                            let sub =
                            {
                                let mut h = HashMap::new();
                                h.insert(name.as_str().to_owned(), Expression::List(func.clone()));
                                h
                            };

                            call_expression(&stack_push(stack, sub), match &func.as_slice()[2]
                            {
                                &Expression::List(ref lambda) => lambda.clone(),
                                _ => return Err(format!("The function in a label must be a list"))
                            }, arguments)
                        }
                        _ => Err(format!("A list is not a valid label"))
                    },
                    name => Err(format!("the atom '{}' is not a way to create a function", name))
                },
                _ => Err(format!("A list cannot be the first element of a function"))
            },
            n => Err(format!("A list with {} elements cannot be a function", n))
        }
    }

    fn stack_lookup(stack: &[Rc<HashMap<String, Expression>>], name: &str) -> Option<Expression>
    {
        for scope in stack.iter().rev()
        {
            if let Some(expr) = scope.get(name)
            {
                return Some(expr.clone())
            }
        }
        return None
    }

    fn stack_push(stack: &[Rc<HashMap<String, Expression>>], names: HashMap<String, Expression>) -> Vec<Rc<HashMap<String, Expression>>>
    {
        let mut tmp = stack.to_vec();
        tmp.push(Rc::new(names));
        tmp
    }

    eval_inner(input, &Vec::new())
}

#[cfg(test)]
mod test
{
    use super::eval;

    use expression::*;
    use token::*;

    #[test]
    fn quote()
    {
        let input = "'t";

        let expected = Expression::Atom(Atom::new("t"));

        let actual = eval(Expression::parse(Token::lex(input)).unwrap()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn atom()
    {
        let input_1 = "(atom 't)";
        let input_2 = "(atom '())";

        let expected_1 = Expression::Atom(Atom::new("t"));
        let expected_2 = Expression::List(List::new(vec![]));

        let actual_1 = eval(Expression::parse(Token::lex(input_1)).unwrap()).unwrap();
        let actual_2 = eval(Expression::parse(Token::lex(input_2)).unwrap()).unwrap();

        assert_eq!(expected_1, actual_1);
        assert_eq!(expected_2, actual_2);
    }
    
    #[test]
    fn eq()
    {
        let input_1 = "(eq 't 't)";
        let input_2 = "(eq '() '())";
        let input_3 = "(eq 't '())";

        let expected_1 = Expression::Atom(Atom::new("t"));
        let expected_2 = Expression::Atom(Atom::new("t"));
        let expected_3 = Expression::List(List::new(vec![]));

        let actual_1 = eval(Expression::parse(Token::lex(input_1)).unwrap()).unwrap();
        let actual_2 = eval(Expression::parse(Token::lex(input_2)).unwrap()).unwrap();
        let actual_3 = eval(Expression::parse(Token::lex(input_3)).unwrap()).unwrap();

        assert_eq!(expected_1, actual_1);
        assert_eq!(expected_2, actual_2);
        assert_eq!(expected_3, actual_3);
    }

    #[test]
    fn car()
    {
        let input = "(car '(a b c))";

        let expected = Expression::Atom(Atom::new("a"));

        let actual = eval(Expression::parse(Token::lex(input)).unwrap()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn cdr()
    {
        let input = "(cdr '(a b c))";

        let expected = Expression::List(List::new(vec![
            Expression::Atom(Atom::new("b")),
            Expression::Atom(Atom::new("c"))
        ]));

        let actual = eval(Expression::parse(Token::lex(input)).unwrap()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn cons()
    {
        let input = "(cons 'a '(b c))";

        let expected = Expression::List(List::new(vec![
            Expression::Atom(Atom::new("a")),
            Expression::Atom(Atom::new("b")),
            Expression::Atom(Atom::new("c"))
        ]));

        let actual = eval(Expression::parse(Token::lex(input)).unwrap()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn cond()
    {
        let input_1 = "(cond ('() 'a) ('a 'b))";
        let input_2 = "(cond ('t 'a) ('() 'b))";

        let expected_1 = Expression::Atom(Atom::new("b"));
        let expected_2 = Expression::Atom(Atom::new("a"));

        let actual_1 = eval(Expression::parse(Token::lex(input_1)).unwrap()).unwrap();
        let actual_2 = eval(Expression::parse(Token::lex(input_2)).unwrap()).unwrap();

        assert_eq!(expected_1, actual_1);
        assert_eq!(expected_2, actual_2);
    }

    #[test]
    fn lambda()
    {
        let input = "((lambda (x) (cons x '(b c))) 'a)";

        let expected = Expression::List(List::new(vec![
            Expression::Atom(Atom::new("a")),
            Expression::Atom(Atom::new("b")),
            Expression::Atom(Atom::new("c"))
        ]));

        let actual = eval(Expression::parse(Token::lex(input)).unwrap()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn macro_()
    {
        let input = "((macro (x) (cons x '(b c))) a)";

        let expected = Expression::List(List::new(vec![
            Expression::Atom(Atom::new("a")),
            Expression::Atom(Atom::new("b")),
            Expression::Atom(Atom::new("c"))
        ]));

        let actual = eval(Expression::parse(Token::lex(input)).unwrap()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn label()
    {
        let input = "((label f (lambda (x) (cond ((atom x) (f '())) ('t 'b)))) 'a)";

        let expected = Expression::Atom(Atom::new("b"));

        let actual = eval(Expression::parse(Token::lex(input)).unwrap()).unwrap();

        assert_eq!(expected, actual);
    }
}