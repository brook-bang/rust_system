use std::fmt;
use std::result::Result;

use super::{ast::Node, token::{OperPrec, Token}, tokenizer::Tokenizer};


pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(expr: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Tokenizer::new(expr);
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid charater".into())),
        };
        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
        })
    }

    pub fn parse(&mut self) -> Result<Node,>
}

impl<'a> Parser<'a> {
    fn get_next_token(&mut self) -> Result<(),ParseError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        self.current_token = next_token;
        Ok(())
    }

    fn generate_ast(&mut self,oper_prec: OperPrec) -> Result<Node, ParseError> {
        let mut letf_expr = self.parse
    }

    fn parse_number(&mut self) -> Result<Node,ParseError> {
        let token = self.current_token.clone();
        match token {
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Num(i) => {
                self.get_next_token()
            }

        }
    }
    
}

#[derive(Debug)]
pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            self::ParseError::UnableToParse(e) => write!(f, "Error in evaluating {}", e),
            self::ParseError::InvalidOperator(e) => write!(f, "Error in evaluating {}", e),
        }
    }
}

