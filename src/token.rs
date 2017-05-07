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