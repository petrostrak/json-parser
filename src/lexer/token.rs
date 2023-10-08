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
