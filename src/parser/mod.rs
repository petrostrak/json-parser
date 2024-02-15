use std::{collections::HashMap, iter::Peekable};

pub struct LogParser<'a> {
    json_str: &'a str,
}

enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl<'a> LogParser<'a> {
    pub fn new(json: &str) -> LogParser<'a> {
        LogParser { json_str: json }
    }

    pub fn parse(&mut self) -> Option<JsonValue> {
        let mut chars = self.json_str.chars().peekable();
        parse_value(&mut chars)
    }
}

fn parse_value(chars: &mut Peekable<std::str::Chars>) -> Option<JsonValue> {
    match chars.peek() {
        Some(&ch) => match ch {
            'n' => parse_null(chars),
            't' | 'f' => parse_bool(chars),
            '"' => parse_string(chars),
            '[' => parse_array(chars),
            '{' => parse_object(chars),
            _ => parse_number(chars),
        },
        _ => None,
    }
}

fn consume(chars: &mut Peekable<std::str::Chars>, expected_str: &str) -> Option<String> {
    let mut result = String::new();
    for expected_ch in expected_str.chars() {
        if let Some(ch) = chars.next() {
            if ch != expected_ch {
                return None;
            }
            result.push(ch);
        } else {
            return None;
        }
    }
    Some(result)
}

fn parse_null(chars: &mut Peekable<std::str::Chars>) -> Option<JsonValue> {
    consume(chars, "null").map(|_| JsonValue::Null)
}

fn parse_bool(chars: &mut Peekable<std::str::Chars>) -> Option<JsonValue> {
    let value = if consume(chars, "true").is_some() {
        true
    } else if consume(chars, "false").is_some() {
        false
    } else {
        return None;
    };
    Some(JsonValue::Bool(value))
}

fn parse_string(chars: &mut Peekable<std::str::Chars>) -> Option<JsonValue> {
    consume(chars, "\"")?;
    let mut string = String::new();
    while let Some(ch) = chars.next() {
        match ch {
            '"' => return Some(JsonValue::String(string)),
            _ => string.push(ch),
        }
    }
    None
}

fn parse_array(chars: &mut Peekable<std::str::Chars>) -> Option<JsonValue> {
    consume(chars, "[")?;
    let mut array = Vec::new();

    loop {
        match parse_value(chars) {
            Some(value) => {
                array.push(value);
                if consume(chars, ",").is_none() {
                    break;
                }
            }
            None => return None,
        }
    }
    consume(chars, "]")?;
    Some(JsonValue::Array(array))
}
