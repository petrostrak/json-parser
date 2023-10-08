use crate::lexer::token::Token;
use anyhow::Result;
use parser::LogParser;

mod lexer;
mod parser;

fn main() -> Result<()> {
    let json = String::from("{\"name\": \"Petros\", \"age\": 37, \"married\": false}");

    let mut l = lexer::Lexer::new(json.chars().collect());
    l.read_char();

    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let token = l.next_token();
        if token == lexer::token::Token::EOF {
            break;
        } else {
            tokens.push(token);
        }
    }

    let kv = LogParser::parse(tokens)?;
    println!("{:?}", kv);

    Ok(())
}
