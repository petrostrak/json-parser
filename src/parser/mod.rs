use anyhow::{anyhow, Ok, Result};
use std::{collections::HashMap, iter::Peekable};

use crate::lexer::token::{Token, Tokens};

pub struct LogParser {
    tokens: Tokens,
    kv: HashMap<String, String>,
}

impl LogParser {
    pub fn new(tokens: Tokens) -> LogParser {
        LogParser {
            tokens,
            kv: HashMap::new(),
        }
    }

    pub fn parse(&mut self) -> Result<HashMap<String, String>> {
        while let Some(_token) = self.tokens.iter().next() {
            self.parse_token(&mut self.tokens.clone().peekable())?;
        }

        Ok(self.kv.clone())
    }

    // parse_token takes the json as a list of tokens and validates them.
    fn parse_token(&mut self, token: &mut Peekable<Tokens>) -> Result<()> {
        match token.peek() {
            Some(Token::LBRACE(_)) => {
                token.next();
                self.parse_object(token)?;
            }
            _ => return Err(anyhow!("missing '{{' at the beginning of json")),
        }
        Ok(())
    }

    // parse_object takes an object and validates it.
    // e.g. "name": "Peter"
    fn parse_object(&mut self, token: &mut Peekable<Tokens>) -> Result<()> {
        match token.peek() {
            Some(Token::QUOTATION(_)) => {
                token.next();
                self.parse_ident(token)?;
            }
            _ => return Err(anyhow!("missing '\"' at the beginning of json")),
        }

        Ok(())
    }

    // parse_ident pushes the value the IDENT() holds into the kv hash table and
    // also checks it the next token is QUOTATION().
    fn parse_ident(&mut self, token: &mut Peekable<Tokens>) -> Result<()> {
        match token.peek() {
            Some(Token::IDENT(_)) => {
                token.next();
                if token.peek() != Some(&Token::QUOTATION('"')) {
                    return Err(anyhow!("Identifier must end with QUOTATION"));
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_kv_for_key(&mut self, token: &mut Peekable<Tokens>) -> Result<()> {
        let mut key = String::new();
        loop {
            match token.peek() {
                Some(Token::IDENT(text)) => {
                    key.push_str(text);
                }
                Some(Token::COLON(_)) => {
                    token.next();
                    break;
                }
                Some(Token::ILLEGAL) => {
                    return Err(anyhow!("unexpected character in key value",));
                }
                None => {
                    return Err(anyhow!("key should be followed by ':' and a value"));
                }
                _ => todo!(),
            }
        }

        self.parse_kv_for_value(key, token)?;

        Ok(())
    }

    fn parse_kv_for_value(&mut self, key: String, token: &mut Peekable<Tokens>) -> Result<()> {
        let mut value = String::new();
        loop {
            match token.peek() {
                Some(Token::IDENT(text)) => {
                    value.push_str(text);
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
