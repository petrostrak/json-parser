#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENT(Vec<char>),
    COLON(char),
    COMMA(char),
    LBRACE(char),
    RBRACE(char),
    TRUE,
    FALSE,
}

pub fn get_keyword_token(ident: &Vec<char>) -> Result<Token, String> {
    let identifier = ident.into_iter().collect::<String>();
    match &identifier[..] {
        "true" => Ok(Token::TRUE),
        "false" => Ok(Token::FALSE),
        _ => Err(String::from("Not a keyword")),
    }
}
