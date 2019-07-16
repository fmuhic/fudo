#[allow(dead_code)]
pub enum TokenType {                                   
  // Single-character tokens.                      
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star, 

  // One or two character tokens.                  
  Bang, BangEqual,                                
  Equal, EqualEqual,                              
  Greater, GreaterEqual,                          
  Less, LessEqual,                                

  // Literals.                                     
  Identifier, StringLiteral, NumberLiteral,                      

  // Keywords.                                     
  And, Class, Else, False, Fun, For, If, Nil, Or,  
  Print, Return, Super, This, True, Var, While,    

  Eof                                              
}

pub struct Token {
    pub literal: String,
    pub token_type: TokenType,
    pub line: u32
}

impl Token {
    pub fn new(literal: String, token_type: TokenType, line: u32) -> Token {
        Token{literal: literal, token_type: token_type, line: line}
    }
}
