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

pub fn get_keyword_token(ident: &str) -> Result<Token, String> {
    match ident {
        "true" => Ok(Token::TRUE),
        "false" => Ok(Token::FALSE),
        _ => Err(String::from("Not a keyword")),
    }
}
