use crate::lexer::token::Tokens;
use anyhow::Result;
// use parser::LogParser;

mod lexer;
mod parser;

fn main() -> Result<()> {
    let json = String::from("{\"name\": \"Petros\", \"age\": 37, \"married\": false}");

    let mut l = lexer::Lexer::new(json.chars().collect());
    l.read_char();

    let mut tokens = Tokens::new(Vec::new());
    loop {
        let token = l.next_token();
        if token == lexer::token::Token::EOF {
            break;
        }
        tokens.list.push(token);
    }

    // let mut parser = LogParser::new(tokens.clone());

    // let kv = parser.parse()?;
    // println!("{:?}", kv);

    let tokens = Tokens::new(tokens.list);
    for token in tokens.iter() {
        println!("{:?}", token);
    }

    Ok(())
}
