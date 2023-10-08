use crate::lexer::token::Token;

mod lexer;
mod parser;

fn main() {
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

    for token in tokens {
        println!("{:?}", token.to_string());
    }
}
