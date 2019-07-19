use super::token::Token;
use super::token::TokenType;

use itertools::MultiPeek;
use std::str::Chars;
use std::iter::Enumerate;
use itertools::multipeek;

pub struct Parser {
    line_number: u32,
    reserved_words: Vec<(TokenType, String)>
}

impl Parser {
    pub fn new() -> Parser {
        let mut language_keywords = Vec::new();
        language_keywords.push((TokenType::And, "and".to_string()));
        language_keywords.push((TokenType::Class, "class".to_string()));
        language_keywords.push((TokenType::Else, "else".to_string()));
        language_keywords.push((TokenType::False, "false".to_string()));
        language_keywords.push((TokenType::Fun, "fun".to_string()));
        language_keywords.push((TokenType::For, "for".to_string()));
        language_keywords.push((TokenType::If, "if".to_string()));
        language_keywords.push((TokenType::Nil, "nil".to_string()));
        language_keywords.push((TokenType::Or, "or".to_string()));
        language_keywords.push((TokenType::Print, "print".to_string()));
        language_keywords.push((TokenType::Return, "return".to_string()));
        language_keywords.push((TokenType::Super, "super".to_string()));
        language_keywords.push((TokenType::This, "this".to_string()));
        language_keywords.push((TokenType::True, "true".to_string()));
        language_keywords.push((TokenType::Var, "var".to_string()));
        language_keywords.push((TokenType::While, "while".to_string()));
        Parser{line_number: 1, reserved_words: language_keywords}
    }

    pub fn parse(&mut self, src: &String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut src_iter = multipeek(src.chars().enumerate());
        while src_iter.peek() != None {
            match self.parse_token(&mut src_iter) {
                Some(token) => {
                    println!("{:?}", token.literal);
                    tokens.push(token);
                },
                None => {
                    println!("Unknown token");
                }
            }
        }
        tokens
    }

    fn parse_token(&mut self, src_iter: &mut MultiPeek<Enumerate<Chars>>) -> Option<Token> {
        match src_iter.next() {
            Some((_index, character)) => {
                match character {
                    ' ' => {
                        println!("found space");
                        None
                    },
                    '\t' => {
                        println!("found tab");
                        None
                    },
                    '\n' => {
                        println!("found new line");
                        self.line_number = self.line_number + 1;
                        None
                    },
                    ';' => Some(Token::new(character.to_string(), TokenType::Semicolon, self.line_number)),
                    '(' => Some(Token::new(character.to_string(), TokenType::LeftParen, self.line_number)),
                    ')' => Some(Token::new(character.to_string(), TokenType::RightParen, self.line_number)),
                    '{' => Some(Token::new(character.to_string(), TokenType::LeftBrace, self.line_number)),
                    '}' => Some(Token::new(character.to_string(), TokenType::RightBrace, self.line_number)),
                    ',' => Some(Token::new(character.to_string(), TokenType::Comma, self.line_number)),
                    '.' => Some(Token::new(character.to_string(), TokenType::Dot, self.line_number)),
                    '-' => Some(Token::new(character.to_string(), TokenType::Minus, self.line_number)),
                    '+' => Some(Token::new(character.to_string(), TokenType::Plus, self.line_number)),
                    ';' => Some(Token::new(character.to_string(), TokenType::Semicolon, self.line_number)),
                    '/' => Some(Token::new(character.to_string(), TokenType::Slash, self.line_number)),
                    '*' => Some(Token::new(character.to_string(), TokenType::Star, self.line_number)),
                    '!' => {
                        match src_iter.peek() {
                            Some((_, next_char)) => {
                                if *next_char == '=' {
                                    src_iter.next();
                                    return Some(Token::new("!=".to_string(), TokenType::BangEqual, self.line_number));
                                }
                            },
                            None => {}
                        };
                        Some(Token::new(character.to_string(), TokenType::Bang, self.line_number))
                    },
                    '=' => {
                        match src_iter.peek() {
                            Some((_, next_char)) => {
                                if *next_char == '=' {
                                    src_iter.next();
                                    return Some(Token::new("==".to_string(), TokenType::EqualEqual, self.line_number));
                                }
                            },
                            None => {}
                        };
                        Some(Token::new(character.to_string(), TokenType::Equal, self.line_number))
                    },
                    '<' => {
                        match src_iter.peek() {
                            Some((_, next_char)) => {
                                if *next_char == '=' {
                                    src_iter.next();
                                    return Some(Token::new("<=".to_string(), TokenType::LessEqual, self.line_number));
                                }
                            },
                            None => {}
                        };
                        Some(Token::new(character.to_string(), TokenType::Less, self.line_number))
                    },
                    '>' => {
                        match src_iter.peek() {
                            Some((_, next_char)) => {
                                if *next_char == '=' {
                                    src_iter.next();
                                    return Some(Token::new(">=".to_string(), TokenType::GreaterEqual, self.line_number));
                                }
                            },
                            None => {}
                        };
                        Some(Token::new(character.to_string(), TokenType::Greater, self.line_number))
                    },
                    '"' => {
                        let token = self.parse_string_literal(src_iter);
                        src_iter.next();
                        token
                    },
                    c if character.is_digit(10) => {
                        self.parse_numeric_literal(c, src_iter)
                    },
                    c if character.is_ascii_alphabetic() => {
                        match self.parse_reserved_word(c, src_iter) {
                            Some(token) => return Some(token),
                            _ => {}
                        }
                        println!("not a keyword");
                        match self.parse_identifier(c, src_iter) {
                            Some(token) => return Some(token),
                            _ => {}
                        }
                        println!("not a identifier");
                        None
                    },
                    _ => {
                        None
                    }
                }
            },
            None => None
        }
    }

    fn parse_numeric_literal(&self, character: char, src_iter: &mut MultiPeek<Enumerate<Chars>>) -> Option<Token> {
        let mut literal: Vec<char> = Vec::new();
        literal.push(character);
        let mut next = src_iter.peek();
        while next != None {
            let (_index, next_char) = next.unwrap();
            if !next_char.is_digit(10) {
                break;
            }
            literal.push(*next_char);
            src_iter.next();
            next = src_iter.peek();
        }
        Some(Token::new(literal.into_iter().collect(), TokenType::NumericLiteral, self.line_number))
    }

    fn parse_string_literal(&self, src_iter: &mut MultiPeek<Enumerate<Chars>>) -> Option<Token> {
        let mut literal: Vec<char> = Vec::new();
        let mut next = src_iter.peek();
        while next != None {
            let (_index, next_char) = next.unwrap();
            if *next_char == '"' {
                return Some(Token::new(literal.into_iter().collect(), TokenType::StringLiteral, self.line_number));
            }
            literal.push(*next_char);
            src_iter.next();
            next = src_iter.peek();
        }
        None
    }

    fn parse_reserved_word(&self, character: char, src_iter: &mut MultiPeek<Enumerate<Chars>>) -> Option<Token> {
        let mut char_buffer: Vec<char> = Vec::new();
        char_buffer.push(character);
        for (token_type, keyword) in &self.reserved_words {
            let mut keywords_match = true;
            while char_buffer.len() < keyword.len() {
                match src_iter.peek() {
                    Some((_, next_char)) => { char_buffer.push(*next_char); },
                    None => { continue; }
                }
            }
            for (i, keyword_letter) in keyword.chars().enumerate() {
                if keyword_letter != char_buffer[i] {
                    keywords_match = false;
                    break;
                }
            }

            if keywords_match {
                for _ in 0..(keyword.len() - 1) {
                    src_iter.next();
                }
                return Some(Token::new(keyword.to_string(), *token_type, self.line_number))
            }
        }
        None
    }

    fn parse_identifier(&self, character: char, src_iter: &mut MultiPeek<Enumerate<Chars>>) -> Option<Token> {
        let mut literal: Vec<char> = Vec::new();
        literal.push(character);
        let mut next = src_iter.peek();
        while next != None {
            let (_index, next_char) = next.unwrap();
            println!("next char is {}", *next_char);
            if !next_char.is_ascii_alphabetic() || self.is_empty_space(*next_char) || *next_char == ';' {
                return Some(Token::new(literal.into_iter().collect(), TokenType::Identifier, self.line_number));
            }
            literal.push(*next_char);
            src_iter.next();
            next = src_iter.peek();
        }
        None       
    }

    fn is_empty_space(&self, c: char) -> bool {
        c == ' ' || c == '\t' || c == '\n'
    }
}
