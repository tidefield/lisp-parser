use std::{fmt::Display, iter::Peekable, str::Chars};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    LeftParen,
    RightParen,
    Identifier(String),
    Number(u64),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Scanner<'a> {
    pub tokens: Vec<Token>,
    source_iter: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            source_iter: code.chars().peekable(),
            tokens: Vec::new(),
        }
    }

    fn identifier(&mut self) {
        let mut name = String::from("");
        while let Some(ch) = self.source_iter.peek()
            && (*ch != ' ' && *ch != ')')
        {
            name.push(*ch);
            self.source_iter.next();
        }
        self.tokens.push(Token::Identifier(name))
    }

    fn number(&mut self) {
        let mut num = String::from("");
        while let Some(ch) = self.source_iter.peek()
            && ch.is_numeric()
        {
            num.push(*ch);
            self.source_iter.next();
        }
        self.tokens.push(Token::Number(num.parse::<u64>().unwrap()))
    }

    fn add_token(&mut self, token_type: Token) {
        self.tokens.push(token_type);
        self.source_iter.next();
    }

    // Use `mut self` instead of `&mut self` to consume the scanner completely
    // so that scan can't be called twice.
    pub fn scan(mut self) -> Vec<Token> {
        while let Some(char) = self.source_iter.peek() {
            match char {
                '(' => self.add_token(Token::LeftParen),
                ')' => self.add_token(Token::RightParen),
                ' ' => {
                    self.source_iter.next();
                }
                '+' => {
                    self.add_token(Token::Identifier(String::from("+")));
                }
                _ => {
                    // TODO: Handle negative number
                    if char.is_numeric() {
                        self.number();
                    } else if char.is_alphabetic() {
                        self.identifier();
                    }
                }
            }
        }
        for token in &self.tokens {
            print!("token {}", token)
        }
        self.tokens
    }
}
