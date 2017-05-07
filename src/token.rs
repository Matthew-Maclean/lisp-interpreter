#[derive(Clone, Debug, PartialEq)]
pub enum Token
{
    Ident(String),
    OpenParen,
    CloseParen,
    Quote,
}

impl Token
{
    pub fn lex(s: &str) -> Vec<Token>
    {
        let mut tokens = Vec::new();

        let mut in_comment = false;
        let mut in_ident = false;
        let mut ident = String::new();

        for c in s.chars()
        {
            if in_comment
            {
                if c == '\n'
                {
                    in_comment = false;
                }
            }
            else
            {
                match c
                {
                    ';' | '(' | ')' | '\'' |
                    ' ' | '\t' | '\r' | '\n' =>
                    {
                        if in_ident
                        {
                            tokens.push(Token::Ident(ident.clone()));
                            ident.clear();
                            in_ident = false;
                        }

                        match c
                        {
                            ';' => in_comment = true,
                            '(' => tokens.push(Token::OpenParen),
                            ')' => tokens.push(Token::CloseParen),
                            '\'' => tokens.push(Token::Quote),
                            _ => {}
                        }
                    },
                    c =>
                    {
                        in_ident = true;
                        ident.push(c);
                    }
                }
            }
        }

        if in_ident
        {
            tokens.push(Token::Ident(ident));
        }

        tokens
    }
}

#[cfg(test)]
mod test
{
    use super::Token;

    #[test]
    fn lexing()
    {
        let input = "(label fac (lambda (n)   ; here we pretend that a number sytem exists
            (cond                             ;
                ((num-eq zero n) (zero))      ; we also pretend that a num-eq function exists, and a zero value
                ('t (num-mul                  ; also the num-mul function
                    n                         ;
                    (fac                      ;
                        (num-sub n one))))))) ; also a one value, and num-sub";
        
        let expected = vec![
            Token::OpenParen,
                Token::Ident("label".to_owned()),
                Token::Ident("fac".to_owned()),
                Token::OpenParen,
                    Token::Ident("lambda".to_owned()),
                    Token::OpenParen,
                        Token::Ident("n".to_owned()),
                    Token::CloseParen,
                    Token::OpenParen,
                        Token::Ident("cond".to_owned()),
                        Token::OpenParen,
                            Token::OpenParen,
                                Token::Ident("num-eq".to_owned()),
                                Token::Ident("zero".to_owned()),
                                Token::Ident("n".to_owned()),
                            Token::CloseParen,
                            Token::OpenParen,
                                Token::Ident("zero".to_owned()),
                            Token::CloseParen,
                        Token::CloseParen,
                        Token::OpenParen,
                            Token::Quote,
                            Token::Ident("t".to_owned()),
                            Token::OpenParen,
                                Token::Ident("num-mul".to_owned()),
                                Token::Ident("n".to_owned()),
                                Token::OpenParen,
                                    Token::Ident("fac".to_owned()),
                                    Token::OpenParen,
                                        Token::Ident("num-sub".to_owned()),
                                        Token::Ident("n".to_owned()),
                                        Token::Ident("one".to_owned()),
                                    Token::CloseParen,
                                Token::CloseParen,
                            Token::CloseParen,
                        Token::CloseParen,
                    Token::CloseParen,
                Token::CloseParen,
            Token::CloseParen,
        ];

        let actual = Token::lex(input);

        assert_eq!(expected, actual);
    }
}