#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    INT(Vec<char>),
    IDENT(Vec<char>),
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

pub fn get_keyword_token(ident: &Vec<char>) -> Token {
    let identifier = ident.into_iter().collect::<String>();
    match &identifier[..] {
        "true" => Token::TRUE,
        "false" => Token::FALSE,
        _ => Token::ILLEGAL,
    }
}
