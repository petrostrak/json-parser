use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,
    INT(u32),
    IDENT(String),
    COLON(char),
    COMMA(char),
    LBRACE(char),
    RBRACE(char),
    LBRACKET(char),
    RBRACKET(char),
    QUOTATION(char),
    TRUE,
    FALSE,
}

#[derive(Clone)]
pub struct Tokens {
    pub list: Vec<Token>,
    index: usize,
}

impl Tokens {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            list: tokens,
            index: 0,
        }
    }

    pub fn iter(&self) -> TokenIterator {
        TokenIterator {
            tokens: self,
            index: 0,
        }
    }
}

impl Iterator for Tokens {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.list.len() {
            let result = Some(&self.list[self.index]);
            self.index += 1;
            result.cloned()
        } else {
            None
        }
    }
}

pub struct TokenIterator<'a> {
    tokens: &'a Tokens,
    index: usize,
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tokens.list.len() {
            let result = Some(&self.tokens.list[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::IDENT(string) => write!(f, "{}", string),
            Token::INT(int) => write!(f, "{}", int),
            Token::COLON(char) => write!(f, "{}", char),
            Token::COMMA(char) => write!(f, "{}", char),
            Token::LBRACE(char) => write!(f, "{}", char),
            Token::RBRACE(char) => write!(f, "{}", char),
            Token::LBRACKET(char) => write!(f, "{}", char),
            Token::RBRACKET(char) => write!(f, "{}", char),
            Token::QUOTATION(char) => write!(f, "{}", char),
            Token::TRUE => write!(f, "true"),
            Token::FALSE => write!(f, "false"),
            _ => Ok(()),
        }
    }
}

pub fn get_keyword_token(ident: &str) -> Result<Token, String> {
    match ident {
        "true" => Ok(Token::TRUE),
        "false" => Ok(Token::FALSE),
        _ => Err(String::from("Not a keyword")),
    }
}
