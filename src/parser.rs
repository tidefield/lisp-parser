/*
 * Grammar:
 *   expression -> atom | list
 *   list -> "(" expression* ")"
 *   atom -> number | identifier
*/

use std::fmt::Display;

use crate::scanner::Token;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(s) => write!(f, "Unexpected token: {}", s),
        }
    }
}

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

// A recursive descent parser
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

    fn consume(&mut self, token: Token) -> Result<(), ParseError> {
        if let Some(current_token) = self.current_token()
            && current_token == token
        {
            self.token_index += 1;
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(token.to_string()))
        }
    }

    fn advance(&mut self) {
        self.token_index += 1;
    }

    fn list(&mut self) -> Result<Vec<Expr>, ParseError> {
        self.consume(Token::LeftParen)?;
        let mut expressions: Vec<Expr> = vec![];
        while let Some(cur_token) = self.current_token()
            && cur_token != Token::RightParen
        {
            expressions.push(self.expression()?);
        }
        self.consume(Token::RightParen)?;
        Ok(expressions)
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        match self.current_token() {
            Some(Token::LeftParen) => {
                let exprs = self.list()?;
                Ok(Expr::List(exprs))
            }
            Some(Token::Identifier(i)) => {
                self.advance();
                Ok(Expr::Atom(AtomValue::String(i)))
            }
            Some(Token::Number(n)) => {
                self.advance();
                Ok(Expr::Atom(AtomValue::Number(n)))
            }
            Some(token) => Err(ParseError::UnexpectedToken(token.to_string())),
            None => Ok(Expr::List(Vec::new())),
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }
}
