use super::token::Token;
use super::token::TokenType;

use itertools::MultiPeek;
use std::str::Chars;
use std::iter::Enumerate;
use itertools::multipeek;

pub struct Parser {
    tokens: Vec<Token>
}

impl Parser {
    pub fn new() -> Parser {
        Parser{tokens: Vec::new()}
    }

    pub fn parse(&mut self, src: &String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut src_iter = multipeek(src.chars().enumerate());
        while src_iter.peek() != None {
            match self.parse_token(&mut src_iter) {
                Some(token) => {
                    tokens.push(token);
                },
                None => {
                    println!("Unknown token");
                }
            }
        }
        tokens
    }

    fn parse_token(&mut self, src_iter: &mut MultiPeek<Enumerate<Chars>>) -> Option<Token>{
        match src_iter.next() {
            Some((_index, chr)) => {
                match chr {
                    '(' => return Some(Token::new(chr.to_string(), TokenType::LeftParen, 1u32)),
                    ')' => return Some(Token::new(chr.to_string(), TokenType::RightParen, 2u32)),
                    _ => return None
                }
            },
            None => {
                None
            }
        }
    }
}

