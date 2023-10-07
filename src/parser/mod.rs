use std::{collections::HashMap, str::Chars, iter::Peekable};

use crate::lexer::token::Token;

pub struct LogParser {
    tokens: Vec<Token>,
    kv: HashMap<String, String>
}

impl LogParser {
    pub fn parse(tokens: Vec<Token>) -> Result<HashMap<String, String>> {
        let mut parser = LogParser {
            tokens,
            kv: Vec::new(),
        };

        for token in tokens {}

        Ok(parser.kv)
    }

    fn parse_token(&mut self, token: &mut Peekable<Chars>) -> Result<()> {
        
        Ok(())
    }
}
