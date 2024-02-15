use crate::lexer::token::Tokens;
use anyhow::Result;
use parser::LogParser;

mod lexer;
mod parser;

fn main() -> Result<()> {
    let json_str = r#"{"name": "Petros", "age": 37, "is_married": false, "grades": [90, 85, 95]}"#;

    let mut l = lexer::Lexer::new(json_str.chars().collect());
    l.read_char();

    let mut tokens = Tokens::new(Vec::new());
    loop {
        let token = l.next_token();
        if token == lexer::token::Token::EOF {
            break;
        }
        tokens.list.push(token);
    }

    let tokens = Tokens::new(tokens.list);
    for token in tokens.iter() {
        println!("{:?}", token);
    }

    if let Some(json_value) = LogParser::new(json_str).parse() {
        println!("{:#?}", json_value)
    } else {
        println!("Failed to parse JSON")
    }

    Ok(())
}
