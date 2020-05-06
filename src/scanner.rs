use std::fmt;

pub struct Scanner<'a> {
    source: &'a str, // The source code
    start: usize,    // The start of the lexeme being examined
    current: usize,  // The character we are currently looking at
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Scanner {
            source: source_code,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '+' => self.make_token(TokenType::Plus),
            '-' => self.make_token(TokenType::Minus),
            '*' => self.make_token(TokenType::Star),
            '/' => self.make_token(TokenType::Slash),
            '!' => {
                let token_type = if self.next_matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.make_token(token_type)
            }
            '=' => {
                let token_type = if self.next_matches('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.make_token(token_type)
            }
            '>' => {
                let token_type = if self.next_matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.make_token(token_type)
            }
            '<' => {
                let token_type = if self.next_matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.make_token(token_type)
            }
            '"' => self.string_token(),
            '0'..='9' => self.number_token(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier_token(),
            _ => self.error_token("Unexpected character."),
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            type_: token_type,
            name: self.source[self.start..self.current].to_string(),
            line: self.line,
        }
    }

    fn error_token(&self, msg: &str) -> Token {
        Token {
            type_: TokenType::Error,
            name: msg.to_string(),
            line: self.line,
        }
    }

    fn string_token(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string");
        }

        // The closing quote
        self.advance();
        self.make_token(TokenType::String)
    }

    fn number_token(&mut self) -> Token {
        while self.peek().is_digit(10) {
            self.advance();
        }
        // look ma', I have a fractional part
        if self.peek() == '.' {
            // Consume the dot
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        self.make_token(TokenType::Number)
    }

    fn identifier_token(&mut self) -> Token {
        while self.peek().is_alphabetic() || self.peek().is_digit(10) || self.peek() == '_' {
            self.advance();
        }

        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        match self.source[self.start..].chars().next().unwrap() {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1..].chars().next().unwrap() {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                        'o' => self.check_keyword(2, 1, "r", TokenType::For),
                        'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            't' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1..].chars().next().unwrap() {
                        'h' => self.check_keyword(2, 2, "is", TokenType::This),
                        'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),

            _ => TokenType::Identifier,
        }
    }

    fn check_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        token_type: TokenType,
    ) -> TokenType {
        if self.current - self.start == start + length
            && self.source[self.start + start..self.current] == *rest
        {
            return token_type;
        }
        TokenType::Identifier
    }

    fn is_at_end(&self) -> bool {
        self.start == self.source.len()
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1..].chars().next().unwrap()
    }

    fn next_matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current..].chars().next().unwrap() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return ();
                    }
                }
                _ => return (),
            }
        }
    }

    fn peek(&self) -> char {
        self.source[self.current..].chars().next().unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current + 1..].chars().next().unwrap()
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One- or two- character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
    DefaultConstructed,
}

#[derive(Clone)]
pub struct Token {
    pub type_: TokenType,
    pub name: String,
    pub line: usize,
}

impl Token {
    pub fn new() -> Self {
        Token {
            type_: TokenType::DefaultConstructed,
            name: "Default constructed Token".to_string(),
            line: 0,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {}", self.type_, self.name)
    }
}
