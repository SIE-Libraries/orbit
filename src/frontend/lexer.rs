#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    EOF,
    Identifier(String),
    LiteralInteger(i64),
    LiteralString(String),

    // Keywords
    Let,
    Mut,
    Fn,
    Const,
    Check,
    Except,
    HashMap,
    Bool,

    // Types
    TypeI8,
    TypeI16,
    TypeI32,
    TypeI64,
    TypeI128,
    TypeF32,
    TypeF64,
    TypeU8Array, // [u8]

    // Directives
    JitBang, // jit!

    // Operators & Punctuation
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,
    Bang,
    Colon,
    SemiColon,
    Arrow, // ->
    Assign, // =
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub line: usize,
    pub col: usize,
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
            line: 1,
            col: 1,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next();
        if let Some(ch) = c {
            if ch == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        c
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    pub fn tokenize(&mut self) -> Vec<TokenInfo> {
        let mut tokens = Vec::new();
        while let Some(&c) = self.peek_char() {
            let start_col = self.col;
            if c.is_whitespace() {
                self.next_char();
                continue;
            }

            if c.is_alphabetic() || c == '_' {
                let mut ident = String::new();
                while let Some(&c) = self.peek_char() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(self.next_char().unwrap());
                    } else {
                        break;
                    }
                }

                // Check for jit!
                if ident == "jit" && self.peek_char() == Some(&'!') {
                    self.next_char();
                    tokens.push(TokenInfo { token: Token::JitBang, line: self.line, col: start_col });
                    continue;
                }

                let token = match ident.as_str() {
                    "let" => Token::Let,
                    "mut" => Token::Mut,
                    "fn" => Token::Fn,
                    "const" => Token::Const,
                    "check" => Token::Check,
                    "except" => Token::Except,
                    "HashMap" => Token::HashMap,
                    "bool" => Token::Bool,
                    "i8" => Token::TypeI8,
                    "i16" => Token::TypeI16,
                    "i32" => Token::TypeI32,
                    "i64" => Token::TypeI64,
                    "i128" => Token::TypeI128,
                    "f32" => Token::TypeF32,
                    "f64" => Token::TypeF64,
                    _ => Token::Identifier(ident),
                };
                tokens.push(TokenInfo { token, line: self.line, col: start_col });
                continue;
            }

            if c.is_digit(10) {
                let mut val = String::new();
                while let Some(&c) = self.peek_char() {
                    if c.is_digit(10) {
                        val.push(self.next_char().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(TokenInfo {
                    token: Token::LiteralInteger(val.parse().unwrap()),
                    line: self.line,
                    col: start_col,
                });
                continue;
            }

            if c == '"' {
                self.next_char(); // Skip opening quote
                let mut s = String::new();
                while let Some(&c) = self.peek_char() {
                    if c == '"' {
                        break;
                    }
                    s.push(self.next_char().unwrap());
                }
                self.next_char(); // Skip closing quote
                tokens.push(TokenInfo { token: Token::LiteralString(s), line: self.line, col: start_col });
                continue;
            }

            // [u8] handling
            if c == '[' {
                self.next_char();
                if self.peek_char() == Some(&'u') {
                    // Peek ahead more? No, let's just be simple for now
                    // In a real lexer we'd be more robust.
                }
                // Check for [u8]
                if self.source[self.source.len() - self.chars.clone().count() ..].starts_with("u8]") {
                    self.next_char(); // u
                    self.next_char(); // 8
                    self.next_char(); // ]
                    tokens.push(TokenInfo { token: Token::TypeU8Array, line: self.line, col: start_col });
                    continue;
                }
                tokens.push(TokenInfo { token: Token::LBracket, line: self.line, col: start_col });
                continue;
            }

            let token = match c {
                '(' => { self.next_char(); Token::LParen },
                ')' => { self.next_char(); Token::RParen },
                '{' => { self.next_char(); Token::LBrace },
                '}' => { self.next_char(); Token::RBrace },
                ']' => { self.next_char(); Token::RBracket },
                ',' => { self.next_char(); Token::Comma },
                '.' => { self.next_char(); Token::Dot },
                '!' => { self.next_char(); Token::Bang },
                ':' => { self.next_char(); Token::Colon },
                ';' => { self.next_char(); Token::SemiColon },
                '<' => { self.next_char(); Token::LessThan },
                '>' => { self.next_char(); Token::GreaterThan },
                '=' => { self.next_char(); Token::Assign },
                '-' => {
                    self.next_char();
                    if self.peek_char() == Some(&'>') {
                        self.next_char();
                        Token::Arrow
                    } else {
                        Token::EOF // Placeholder for Minus
                    }
                }
                _ => {
                    self.next_char();
                    Token::EOF // Placeholder for Unknown
                }
            };
            tokens.push(TokenInfo { token, line: self.line, col: start_col });
        }
        tokens.push(TokenInfo { token: Token::EOF, line: self.line, col: self.col });
        tokens
    }
}
