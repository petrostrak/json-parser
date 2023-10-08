use anyhow::{anyhow, Result};
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
        match token.peek() {
            Some(':') => {
                token.next();
                self.parse_ident(token)?;
            }
            Some(_) => {
                self.parse_kv_for_key(token)?;
            }
            None => {}
        }
        Ok(())
    }

    fn parse_ident(&mut self, token: &mut Peekable<Chars>) -> Result<()> {
        let mut ident = String::new();

        loop {
            match token.peek() {
                Some(' ') => {
                    token.next();
                    break;
                }
                None => {
                    break;
                }
                _ => ident.push(token.next().unwrap()),
            }
        }
        Ok(())
    }

    fn parse_kv_for_key(&mut self, token: &mut Peekable<Chars>) -> Result<()> {
        let mut key = String::new();
        loop {
            match token.peek() {
                Some('a'..='z') => {
                    key.push(token.next().unwrap());
                }
                Some('=') => {
                    token.next();
                    break;
                }
                Some(ch) => {
                    return Err(anyhow!("unexpected character '{}; in key value", ch));
                }
                None => {
                    return Err(anyhow!("key should be followed by '=' and a value"));
                }
            }
        }

        self.parse_kv_for_value(key, token)?;

        Ok(())
    }

    fn parse_kv_for_value(&mut self, key: String, token: &mut Peekable<Chars>) -> Result<()> {
        let mut value = String::new();
        loop {
            match token.peek() {
                Some('a'..='z') => {
                    value.push(token.next().unwrap());
                }
                Some(ch) => {
                    return Err(anyhow!("unexpected character '{}' in value", ch));
                }
                None => {
                    break;
                }
            }
        }

        self.kv.insert(key, value);

        Ok(())
    }
}
