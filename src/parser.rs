/*
 * Grammar:
 *   expression -> atom | list
 *   list -> "(" expression* ")"
 *   atom -> number | identifier
*/

use std::fmt::Display;

use crate::scanner::Token;

#[derive(Debug)]
pub enum AtomValue {
    String(String),
    Number(u64),
}

impl Display for AtomValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtomValue::Number(n) => write!(f, "{}", n),
            AtomValue::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    // https://rust-lang.github.io/rust-clippy/rust-1.93.0/index.html#vec_box
    // No need to use Box here because Vec already stores the data on heap
    List(Vec<Expr>),
    Atom(AtomValue),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Atom(atom) => write!(f, "{}", atom),
            Expr::List(list) => {
                write!(f, "[")?;
                for (i, item) in list.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{}", item)?;
                    } else {
                        write!(f, ", {}", item)?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

pub struct Parser {
    token_index: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            token_index: 0,
        }
    }

    fn current_token(&self) -> Option<Token> {
        self.tokens.get(self.token_index).cloned()
    }

    fn consume(&mut self, token: Token) {
        if let Some(current_token) = self.current_token()
            && current_token == token
        {
            self.token_index += 1;
        } else {
            panic!("Invalid token")
        }
    }

    fn advance(&mut self) {
        self.token_index += 1;
    }

    fn list(&mut self) -> Vec<Expr> {
        self.consume(Token::LeftParen);
        let mut expressions: Vec<Expr> = vec![];
        while let Some(cur_token) = self.current_token()
            && cur_token != Token::RightParen
        {
            expressions.push(self.expression());
        }
        self.consume(Token::RightParen);
        expressions
    }

    fn expression(&mut self) -> Expr {
        if let Some(token) = self.current_token() {
            match token {
                Token::LeftParen => Expr::List(self.list()),
                Token::Identifier(i) => {
                    self.advance();
                    Expr::Atom(AtomValue::String(i))
                }
                Token::Number(n) => {
                    self.advance();
                    Expr::Atom(AtomValue::Number(n))
                }
                _ => {
                    panic!("unexpected token")
                }
            }
        } else {
            Expr::List(Vec::new())
        }
    }

    // Recursive descent parser
    pub fn parse(&mut self) -> Expr {
        self.expression()
    }
}
