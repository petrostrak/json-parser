use anyhow::Result;
use std::{collections::HashMap, iter::Peekable, str::Chars};

use crate::lexer::token::Token;

pub struct LogParser {
    tokens: Vec<Token>,
    kv: HashMap<String, String>,
}

impl LogParser {
    pub fn parse(tokens: Vec<Token>) -> Result<HashMap<String, String>> {
        let mut parser = LogParser {
            tokens,
            kv: HashMap::new(),
        };

        while let Some(token) = parser.tokens.iter().next() {
            parser.parse_token(&mut token.to_string().chars().peekable())?;
        }

        Ok(parser.kv)
    }

    fn parse_token(&mut self, token: &mut Peekable<Chars>) -> Result<()> {
        Ok(())
    }
}
