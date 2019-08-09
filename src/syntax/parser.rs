use super::token::Token;
use super::token::TokenType;

use itertools::MultiPeek;
use std::str::Chars;
use itertools::multipeek;

pub struct Parser {
    line_number: u32,
    column_number: u32,
    reserved_words: Vec<(TokenType, String)>
}

impl Parser {
    pub fn new() -> Parser {
        //Todo(Fudo): Inject this
        let language_keywords = vec![
            (TokenType::And, "and".to_string()),
            (TokenType::Class, "class".to_string()),
            (TokenType::Else, "else".to_string()),
            (TokenType::False, "false".to_string()),
            (TokenType::Fun, "fun".to_string()),
            (TokenType::For, "for".to_string()),
            (TokenType::If, "if".to_string()),
            (TokenType::Nil, "nil".to_string()),
            (TokenType::Or, "or".to_string()),
            (TokenType::Print, "print".to_string()),
            (TokenType::Return, "return".to_string()),
            (TokenType::Super, "super".to_string()),
            (TokenType::This, "this".to_string()),
            (TokenType::True, "true".to_string()),
            (TokenType::Var, "var".to_string()),
            (TokenType::While, "while".to_string()),
        ];
        Parser{line_number: 1, column_number: 0, reserved_words: language_keywords}
    }

    pub fn parse(&mut self, src: &String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut src_iter = multipeek(src.chars());
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

    fn parse_token(&mut self, src_iter: &mut MultiPeek<Chars>) -> Option<Token> {
        match self.advance(src_iter, 1) {
            Some(character) => {
                match character {
                    ' ' | '\t' | '\n'=> {
                        None
                    },
                    ';' => Some(Token::new(TokenType::Semicolon, self.line_number, self.column_number)),
                    '(' => Some(Token::new(TokenType::LeftParen, self.line_number, self.column_number)),
                    ')' => Some(Token::new(TokenType::RightParen, self.line_number, self.column_number)),
                    '{' => Some(Token::new(TokenType::LeftBrace, self.line_number, self.column_number)),
                    '}' => Some(Token::new(TokenType::RightBrace, self.line_number, self.column_number)),
                    ',' => Some(Token::new(TokenType::Comma, self.line_number, self.column_number)),
                    '.' => Some(Token::new(TokenType::Dot, self.line_number, self.column_number)),
                    '-' => Some(Token::new(TokenType::Minus, self.line_number, self.column_number)),
                    '+' => Some(Token::new(TokenType::Plus, self.line_number, self.column_number)),
                    '/' => Some(Token::new(TokenType::Slash, self.line_number, self.column_number)),
                    '*' => Some(Token::new(TokenType::Star, self.line_number, self.column_number)),
                    '!' => {
                        match src_iter.peek() {
                            Some(next_char) => {
                                if *next_char == '=' {
                                    src_iter.next();
                                    return Some(Token::new(TokenType::BangEqual, self.line_number, self.column_number));
                                }
                            },
                            None => {}
                        };
                        Some(Token::new(TokenType::Bang, self.line_number, self.column_number))
                    },
                    '=' => {
                        match src_iter.peek() {
                            Some(next_char) => {
                                if *next_char == '=' {
                                    src_iter.next();
                                    return Some(Token::new(TokenType::EqualEqual, self.line_number, self.column_number));
                                }
                            },
                            None => {}
                        };
                        Some(Token::new(TokenType::Equal, self.line_number, self.column_number))
                    },
                    '<' => {
                        match src_iter.peek() {
                            Some(next_char) => {
                                if *next_char == '=' {
                                    src_iter.next();
                                    return Some(Token::new(TokenType::LessEqual, self.line_number, self.column_number));
                                }
                            },
                            None => {}
                        };
                        Some(Token::new(TokenType::Less, self.line_number, self.column_number))
                    },
                    '>' => {
                        match src_iter.peek() {
                            Some(next_char) => {
                                if *next_char == '=' {
                                    src_iter.next();
                                    return Some(Token::new(TokenType::GreaterEqual, self.line_number, self.column_number));
                                }
                            },
                            None => {}
                        };
                        Some(Token::new(TokenType::Greater, self.line_number, self.column_number))
                    },
                    '"' => {
                        match self.parse_string_literal(src_iter) {
                            Some(literal) => {
                                self.advance(src_iter, (literal.chars().count() + 1) as u32);
                                return Some(Token::new(TokenType::StringLiteral(literal), self.line_number, self.column_number));
                            },
                            None => {
                                //Todo(Fudo): String literal is not closed. Handle this error.
                                None
                            }
                        }
                    },
                    c if character.is_digit(10) => {
                        match self.parse_numeric_literal(c, src_iter) {
                            Some(literal) => {
                                //Todo(Fudo): Invalid token position
                                self.advance(src_iter, (literal.chars().count() - 1) as u32);
                                return Some(Token::new(TokenType::NumericLiteral(literal.parse().unwrap()), self.line_number, self.column_number));
                            }
                            _ => {
                                None
                            }
                        }
                    },
                    c if character.is_ascii_alphabetic() => {
                        match self.parse_reserved_word(c, src_iter) {
                            Some((token_type, token_length)) => {
                                self.advance(src_iter, (token_length - 1) as u32);
                                return Some(Token::new(token_type, self.line_number, self.column_number));
                            },
                            _ => {}
                        }
                        match self.parse_identifier(c, src_iter) {
                            Some(literal) => {
                                self.advance(src_iter, literal.chars().count() as u32);
                                return Some(Token::new(TokenType::Identifier(literal), self.line_number, self.column_number));
                            },
                            _ => {}
                        }
                        None
                    },
                    _ => {
                        None
                    }
                }
            },
            None => {
                 Some(Token::new(TokenType::Eof, self.line_number, self.column_number))
            }
        }
    }

    fn parse_numeric_literal(&mut self, character: char, src_iter: &mut MultiPeek<Chars>) -> Option<String> {
        let mut literal: Vec<char> = Vec::new();
        literal.push(character);
        let mut next = src_iter.peek();
        while next != None {
            let next_char = next.unwrap();
            if !next_char.is_digit(10) && *next_char != '.' {
                break;
            }
            literal.push(*next_char);
            next = src_iter.peek();
        }
        Some(literal.into_iter().collect())
    }

    fn parse_string_literal(&mut self, src_iter: &mut MultiPeek<Chars>) -> Option<String> {
        let mut literal: Vec<char> = Vec::new();
        let mut next = src_iter.peek();
        while next != None {
            let next_char = next.unwrap();
            if *next_char == '"' {
                return Some(literal.into_iter().collect());
            }
            literal.push(*next_char);
            next = src_iter.peek();
        }
        None
    }

    fn parse_reserved_word(&mut self, character: char, src_iter: &mut MultiPeek<Chars>) -> Option<(TokenType, u32)> {
        for (token_type, keyword) in &self.reserved_words {
            src_iter.reset_peek();
            let mut current_char = character;
            let mut keywords_match = true;
            for keyword_letter in keyword.chars() {
                if keyword_letter != current_char {
                    keywords_match = false;
                    break;
                }
                match src_iter.peek() {
                    Some(next_char) => { current_char = *next_char },
                    None => { keywords_match = false; continue; }
                }
            }

            if keywords_match {
                if !self.is_identifier(current_char) {
                    return Some((token_type.clone(), keyword.chars().count() as u32));
                }
            }
        }
        None
    }

    fn parse_identifier(&mut self, character: char, src_iter: &mut MultiPeek<Chars>) -> Option<String> {
        src_iter.reset_peek();
        let mut literal: Vec<char> = Vec::new();
        literal.push(character);
        let mut next = src_iter.peek();
        while next != None {
            let next_char = next.unwrap();
            if !self.is_identifier(*next_char) {
                break
            }
            literal.push(*next_char);
            next = src_iter.peek();
        }
        return Some(literal.into_iter().collect());
    }

    fn advance(&mut self, src_iter: &mut MultiPeek<Chars>, amount: u32) -> Option<char> {
        let mut next_char = None;
        for _ in 0..amount {
            next_char = src_iter.next();
            match next_char {
                Some(c) => {
                    if c == '\n' {
                        self.line_number = self.line_number + 1;
                        self.column_number = 0;
                    }
                    else {
                        self.column_number = self.column_number + 1;
                    }
                }
                _ => {}
            }
        }
        next_char
    }

    fn is_identifier(&self, c: char) -> bool {
        c == '_' || c.is_digit(10) || c.is_ascii_alphabetic()
    }
}
