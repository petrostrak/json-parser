mod lexer;

fn main() {
    let json = String::from("{\"name\": \"Petros\", \"age\": 37, \"married\": false}");

    let mut l = lexer::Lexer::new(json.chars().collect());
    l.read_char();
    loop {
        let token = l.next_token();
        if token == lexer::token::Token::EOF {
            break;
        } else {
            println!("{token:?}");
        }
    }
    println!("{} {} {}", char::from(l.ch), l.position, l.read_position);
}
