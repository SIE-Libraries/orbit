/// Token types for the Spaceship language
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    True,
    False,

    // Keywords
    Let,
    Const,
    Fn,
    If,
    Else,
    While,
    For,
    Return,
    Check,
    Except,
    Process,
    Jit,
    Fn,
    
    // Types
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Not,
    Dot,
    Colon,
    Semicolon,
    Comma,
    Arrow,      // ->
    FatArrow,   // =>
    Pipe,       // |
    Exclaim,    // !
    Question,   // ?

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // Identifiers
    Identifier(String),

    // Special
    Eof,
    Unknown(char),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Get the current character without consuming it
    fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    /// Get the next character without consuming it
    fn peek_next(&self) -> Option<char> {
        if self.position + 1 < self.input.len() {
            Some(self.input[self.position + 1])
        } else {
            None
        }
    }

    /// Consume and return the current character
    fn next_char(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            self.position += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    /// Skip whitespace and comments
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    self.next_char();
                }
                '/' if self.peek_next() == Some('/') => {
                    // Single-line comment
                    self.next_char();
                    self.next_char();
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.next_char();
                    }
                }
                '/' if self.peek_next() == Some('*') => {
                    // Multi-line comment
                    self.next_char();
                    self.next_char();
                    while let Some(c) = self.peek() {
                        if c == '*' && self.peek_next() == Some('/') {
                            self.next_char();
                            self.next_char();
                            break;
                        }
                        self.next_char();
                    }
                }
                _ => break,
            }
        }
    }

    /// Read a string literal (double-quoted)
    fn read_string(&mut self) -> String {
        self.next_char(); // consume opening quote
        let mut result = String::new();
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.next_char();
                break;
            } else if ch == '\\' {
                self.next_char();
                if let Some(escaped) = self.peek() {
                    match escaped {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        _ => result.push(escaped),
                    }
                    self.next_char();
                }
            } else {
                result.push(ch);
                self.next_char();
            }
        }
        result
    }

    /// Read a number (integer or float)
    fn read_number(&mut self) -> TokenType {
        let mut num_str = String::new();
        let mut is_float = false;

        while let Some(ch) = self.peek() {
            if ch.is_numeric() {
                num_str.push(ch);
                self.next_char();
            } else if ch == '.' && !is_float {
                is_float = true;
                num_str.push(ch);
                self.next_char();
            } else {
                break;
            }
        }

        if is_float {
            TokenType::Float(num_str.parse().unwrap_or(0.0))
        } else {
            TokenType::Integer(num_str.parse().unwrap_or(0))
        }
    }

    /// Read an identifier or keyword
    fn read_identifier(&mut self) -> TokenType {
        let mut ident = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.next_char();
            } else {
                break;
            }
        }

        match ident.as_str() {
            "let" => TokenType::Let,
            "const" => TokenType::Const,
            "fn" => TokenType::Fn,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "return" => TokenType::Return,
            "check" => TokenType::Check,
            "except" => TokenType::Except,
            "Process" => TokenType::Process,
            "jit" => TokenType::Jit,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "bool" => TokenType::Bool,
            "i8" => TokenType::I8,
            "i16" => TokenType::I16,
            "i32" => TokenType::I32,
            "i64" => TokenType::I64,
            "i128" => TokenType::I128,
            "u8" => TokenType::U8,
            "u16" => TokenType::U16,
            "u32" => TokenType::U32,
            "u64" => TokenType::U64,
            "u128" => TokenType::U128,
            "f32" => TokenType::F32,
            "f64" => TokenType::F64,
            _ => TokenType::Identifier(ident),
        }
    }

    /// Get the next token
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let line = self.line;
        let column = self.column;

        match self.peek() {
            None => Token {
                token_type: TokenType::Eof,
                line,
                column,
            },
            Some(ch) => {
                let token_type = match ch {
                    '(' => {
                        self.next_char();
                        TokenType::LeftParen
                    }
                    ')' => {
                        self.next_char();
                        TokenType::RightParen
                    }
                    '{' => {
                        self.next_char();
                        TokenType::LeftBrace
                    }
                    '}' => {
                        self.next_char();
                        TokenType::RightBrace
                    }
                    '[' => {
                        self.next_char();
                        TokenType::LeftBracket
                    }
                    ']' => {
                        self.next_char();
                        TokenType::RightBracket
                    }
                    '+' => {
                        self.next_char();
                        TokenType::Plus
                    }
                    '-' => {
                        self.next_char();
                        if self.peek() == Some('>') {
                            self.next_char();
                            TokenType::Arrow
                        } else {
                            TokenType::Minus
                        }
                    }
                    '*' => {
                        self.next_char();
                        TokenType::Star
                    }
                    '/' => {
                        self.next_char();
                        TokenType::Slash
                    }
                    '%' => {
                        self.next_char();
                        TokenType::Percent
                    }
                    '=' => {
                        self.next_char();
                        if self.peek() == Some('=') {
                            self.next_char();
                            TokenType::EqualEqual
                        } else if self.peek() == Some('>') {
                            self.next_char();
                            TokenType::FatArrow
                        } else {
                            TokenType::Equal
                        }
                    }
                    '!' => {
                        self.next_char();
                        if self.peek() == Some('=') {
                            self.next_char();
                            TokenType::NotEqual
                        } else {
                            TokenType::Exclaim
                        }
                    }
                    '<' => {
                        self.next_char();
                        if self.peek() == Some('=') {
                            self.next_char();
                            TokenType::LessEqual
                        } else {
                            TokenType::Less
                        }
                    }
                    '>' => {
                        self.next_char();
                        if self.peek() == Some('=') {
                            self.next_char();
                            TokenType::GreaterEqual
                        } else {
                            TokenType::Greater
                        }
                    }
                    '&' => {
                        self.next_char();
                        if self.peek() == Some('&') {
                            self.next_char();
                            TokenType::And
                        } else {
                            TokenType::Unknown(ch)
                        }
                    }
                    '|' => {
                        self.next_char();
                        if self.peek() == Some('|') {
                            self.next_char();
                            TokenType::Or
                        } else {
                            TokenType::Pipe
                        }
                    }
                    '.' => {
                        self.next_char();
                        TokenType::Dot
                    }
                    ':' => {
                        self.next_char();
                        TokenType::Colon
                    }
                    ';' => {
                        self.next_char();
                        TokenType::Semicolon
                    }
                    ',' => {
                        self.next_char();
                        TokenType::Comma
                    }
                    '?' => {
                        self.next_char();
                        TokenType::Question
                    }
                    '"' => self.read_string(),
                    _ if ch.is_numeric() => self.read_number(),
                    _ if ch.is_alphabetic() || ch == '_' => self.read_identifier(),
                    _ => {
                        self.next_char();
                        TokenType::Unknown(ch)
                    }
                };

                Token {
                    token_type,
                    line,
                    column,
                }
            }
        }
    }

    /// Tokenize the entire input into a vector of tokens
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            let is_eof = matches!(token.token_type, TokenType::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_keywords() {
        let mut lexer = Lexer::new("let const fn if else");
        let tokens = lexer.tokenize();
        assert_eq!(tokens.len(), 6); // 5 keywords + EOF
        assert!(matches!(tokens[0].token_type, TokenType::Let));
        assert!(matches!(tokens[1].token_type, TokenType::Const));
        assert!(matches!(tokens[2].token_type, TokenType::Fn));
        assert!(matches!(tokens[3].token_type, TokenType::If));
        assert!(matches!(tokens[4].token_type, TokenType::Else));
    }

    #[test]
    fn test_tokenize_numbers() {
        let mut lexer = Lexer::new("42 3.14 100");
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].token_type, TokenType::Integer(42)));
        assert!(matches!(tokens[1].token_type, TokenType::Float(_)));
        assert!(matches!(tokens[2].token_type, TokenType::Integer(100)));
    }

    #[test]
    fn test_tokenize_strings() {
        let mut lexer = Lexer::new(r#""hello world""#);
        let tokens = lexer.tokenize();
        match &tokens[0].token_type {
            TokenType::String(s) => assert_eq!(s, "hello world"),
            _ => panic!("Expected string token"),
        }
    }

    #[test]
    fn test_tokenize_types() {
        let mut lexer = Lexer::new("i32 i64 f64 bool");
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].token_type, TokenType::I32));
        assert!(matches!(tokens[1].token_type, TokenType::I64));
        assert!(matches!(tokens[2].token_type, TokenType::F64));
        assert!(matches!(tokens[3].token_type, TokenType::Bool));
    }

    #[test]
    fn test_tokenize_operators() {
        let mut lexer = Lexer::new("+ - * / == != < > <= >=");
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].token_type, TokenType::Plus));
        assert!(matches!(tokens[1].token_type, TokenType::Minus));
        assert!(matches!(tokens[2].token_type, TokenType::Star));
        assert!(matches!(tokens[3].token_type, TokenType::Slash));
        assert!(matches!(tokens[4].token_type, TokenType::EqualEqual));
        assert!(matches!(tokens[5].token_type, TokenType::NotEqual));
    }

    #[test]
    fn test_tokenize_variable_declaration() {
        let mut lexer = Lexer::new("let x: i32 = 42;");
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].token_type, TokenType::Let));
        match &tokens[1].token_type {
            TokenType::Identifier(name) => assert_eq!(name, "x"),
            _ => panic!("Expected identifier"),
        }
        assert!(matches!(tokens[2].token_type, TokenType::Colon));
        assert!(matches!(tokens[3].token_type, TokenType::I32));
        assert!(matches!(tokens[4].token_type, TokenType::Equal));
        assert!(matches!(tokens[5].token_type, TokenType::Integer(42)));
        assert!(matches!(tokens[6].token_type, TokenType::Semicolon));
    }

    #[test]
    fn test_tokenize_function_declaration() {
        let mut lexer = Lexer::new("fn add(a: i32, b: i32) -> i32 { }");
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].token_type, TokenType::Fn));
        match &tokens[1].token_type {
            TokenType::Identifier(name) => assert_eq!(name, "add"),
            _ => panic!("Expected function name"),
        }
        assert!(matches!(tokens[2].token_type, TokenType::LeftParen));
    }

    #[test]
    fn test_skip_comments() {
        let mut lexer = Lexer::new("let x // this is a comment\n let y;");
        let tokens = lexer.tokenize();
        // Should skip the comment and correctly parse both let statements
        assert!(matches!(tokens[0].token_type, TokenType::Let));
        assert!(matches!(tokens[3].token_type, TokenType::Let));
    }

    #[test]
    fn test_multiline_comments() {
        let mut lexer = Lexer::new("let /* comment */ x;");
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].token_type, TokenType::Let));
        match &tokens[1].token_type {
            TokenType::Identifier(name) => assert_eq!(name, "x"),
            _ => panic!("Expected identifier"),
        }
    }

    #[test]
    fn test_error_handling_syntax() {
        let mut lexer = Lexer::new("fn fail() -> !i32 { }");
        let tokens = lexer.tokenize();
        // Should tokenize the ! character
        let has_exclaim = tokens
            .iter()
            .any(|t| matches!(t.token_type, TokenType::Exclaim));
        assert!(has_exclaim);
    }
}
