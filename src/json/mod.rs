use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Json {
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

impl Json {
    pub fn parse(input: &str) -> Result<Self, String> {
        Parser::new(input).parse()
    }

    pub fn as_object(&self) -> Option<&HashMap<String, Json>> {
        match self {
            Json::Object(o) => Some(o),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Json>> {
        match self {
            Json::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Json::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_usize(&self) -> Option<usize> {
        self.as_f64().map(|n| n as usize)
    }

    pub fn get(&self, key: &str) -> Option<&Json> {
        self.as_object()?.get(key)
    }
}

struct Parser<'a> {
    input: &'a [u8],
    index: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            index: 0,
        }
    }

    fn parse(&mut self) -> Result<Json, String> {
        self.skip_ws();
        self.parse_value()
    }

    fn skip_ws(&mut self) {
        while self.index < self.input.len()
            && matches!(self.input[self.index], b' ' | b'\n' | b'\r' | b'\t')
        {
            self.index += 1;
        }
    }

    fn peek(&self) -> u8 {
        self.input[self.index]
    }

    fn advance(&mut self) -> u8 {
        let ch = self.input[self.index];
        self.index += 1;
        ch
    }

    fn parse_value(&mut self) -> Result<Json, String> {
        self.skip_ws();
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
            _ => Err("Invalid JSON".into()),
        }
    }

    fn parse_object(&mut self) -> Result<Json, String> {
        self.advance();
        let mut map = HashMap::new();

        loop {
            self.skip_ws();
            if self.peek() == b'}' {
                self.advance();
                break;
            }

            let key = self.parse_string()?;
            self.skip_ws();
            if self.advance() != b':' {
                return Err("Expected ':'".into());
            }

            map.insert(key, self.parse_value()?);
            self.skip_ws();

            match self.peek() {
                b',' => {
                    self.advance();
                }
                b'}' => {
                    self.advance();
                    break;
                }
                _ => return Err("Expected ',' or '}'".into()),
            }
        }

        Ok(Json::Object(map))
    }

    fn parse_array(&mut self) -> Result<Json, String> {
        self.advance();
        let mut arr = Vec::new();

        loop {
            self.skip_ws();
            if self.peek() == b']' {
                self.advance();
                break;
            }

            arr.push(self.parse_value()?);
            self.skip_ws();

            match self.peek() {
                b',' => {
                    self.advance();
                }
                b']' => {
                    self.advance();
                    break;
                }
                _ => return Err("Expected ',' or ']'".into()),
            }
        }

        Ok(Json::Array(arr))
    }

    fn parse_string(&mut self) -> Result<String, String> {
        if self.advance() != b'"' {
            return Err("Expected '\"'".into());
        }

        let start = self.index;
        while self.index < self.input.len() && self.input[self.index] != b'"' {
            self.index += 1;
        }

        if self.index >= self.input.len() {
            return Err("Unterminated string".into());
        }

        let s = std::str::from_utf8(&self.input[start..self.index])
            .map_err(|_| "Invalid UTF-8")?
            .to_string();

        self.advance();
        Ok(s)
    }

    fn parse_number(&mut self) -> Result<f64, String> {
        let start = self.index;
        while self.index < self.input.len()
            && matches!(self.peek(), b'0'..=b'9' | b'.' | b'-' | b'+' | b'e' | b'E')
        {
            self.index += 1;
        }

        std::str::from_utf8(&self.input[start..self.index])
            .ok()
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| "Invalid number".into())
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
