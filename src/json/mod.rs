use std::collections::HashMap;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Json {
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

pub struct Parser<'a> {
    input: &'a [u8],
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            index: 0,
        }
    }

    #[inline]
    fn peek(&self) -> u8 {
        self.input[self.index]
    }

    #[inline]
    fn advance(&mut self) -> u8 {
        let ch = self.input[self.index];
        self.index += 1;
        ch
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        while self.index < self.input.len() {
            match self.peek() {
                b' ' | b'\n' | b'\r' | b'\t' => {
                    self.index += 1;
                }

                _ => break,
            }
        }
    }

    pub fn parse(&mut self) -> Result<Json, String> {
        self.skip_whitespace();
        self.parse_value()
    }

    fn parse_value(&mut self) -> Result<Json, String> {
        self.skip_whitespace();

        match self.peek() {
            b'{' => self.parse_object(),
            b'[' => self.parse_array(),
            b'"' => Ok(Json::String(self.parse_string()?)),
            b't' => {
                self.expect(b"true")?;
                Ok(Json::Bool(true))
            }

            b'f' => {
                self.expect(b"false")?;
                Ok(Json::Bool(false))
            }

            b'n' => {
                self.expect(b"null")?;
                Ok(Json::Null)
            }

            b'-' | b'0'..=b'9' => Ok(Json::Number(self.parse_number()?)),

            _ => Err("Invalid JSON value".into()),
        }
    }

    fn parse_object(&mut self) -> Result<Json, String> {
        self.advance();

        let mut object = HashMap::new();

        loop {
            self.skip_whitespace();

            if self.peek() == b'}' {
                self.advance();
                break;
            }

            let key = self.parse_string()?;

            self.skip_whitespace();

            if self.advance() != b':' {
                return Err("Expected ':'".into());
            }

            let value = self.parse_value()?;

            object.insert(key, value);

            self.skip_whitespace();

            match self.peek() {
                b',' => {
                    self.advance();
                }

                b'}' => {
                    self.advance();
                    break;
                }

                _ => {
                    return Err("Expected ',' or '}'".into());
                }
            }
        }

        Ok(Json::Object(object))
    }

    fn parse_array(&mut self) -> Result<Json, String> {
        self.advance();

        let mut array = Vec::new();

        loop {
            self.skip_whitespace();

            if self.peek() == b']' {
                self.advance();
                break;
            }

            array.push(self.parse_value()?);

            self.skip_whitespace();

            match self.peek() {
                b',' => {
                    self.advance();
                }

                b']' => {
                    self.advance();
                    break;
                }

                _ => {
                    return Err("Expected ',' or ']'".into());
                }
            }
        }

        Ok(Json::Array(array))
    }

    fn parse_string(&mut self) -> Result<String, String> {
        if self.advance() != b'"' {
            return Err("Expected string".into());
        }

        let start = self.index;

        while self.peek() != b'"' {
            self.index += 1;

            if self.index >= self.input.len() {
                return Err("Unterminated string".into());
            }
        }

        let end = self.index;

        self.advance();

        Ok(std::str::from_utf8(&self.input[start..end])
            .unwrap()
            .to_string())
    }

    fn parse_number(&mut self) -> Result<f64, String> {
        let start = self.index;

        while self.index < self.input.len() {
            match self.peek() {
                b'0'..=b'9' | b'.' | b'-' | b'+' | b'e' | b'E' => {
                    self.index += 1;
                }

                _ => break,
            }
        }

        let s = std::str::from_utf8(&self.input[start..self.index]).unwrap();

        s.parse::<f64>().map_err(|_| "Invalid number".into())
    }

    fn expect(&mut self, expected: &[u8]) -> Result<(), String> {
        for &b in expected {
            if self.advance() != b {
                return Err("Unexpected token".into());
            }
        }

        Ok(())
    }
}

pub fn obj<'a>(v: &'a Json) -> Option<&'a HashMap<String, Json>> {
    match v {
        Json::Object(o) => Some(o),
        _ => None,
    }
}

pub fn arr<'a>(v: &'a Json) -> Option<&'a Vec<Json>> {
    match v {
        Json::Array(a) => Some(a),
        _ => None,
    }
}

pub fn num(v: &Json) -> Option<f64> {
    match v {
        Json::Number(n) => Some(*n),
        _ => None,
    }
}

pub fn str(v: &Json) -> Option<&str> {
    match v {
        Json::String(s) => Some(s),
        _ => None,
    }
}

pub fn get<'a>(obj: &'a HashMap<String, Json>, key: &str) -> Option<&'a Json> {
    obj.get(key)
}
