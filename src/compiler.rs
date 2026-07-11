use crate::lexer::{Token, TypeNode, Expression, Statement, VarDecl};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    // --- Core Navigation Infrastructure ---

    fn peek(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) -> Token {
        let current = self.peek().clone();
        if current != Token::EOF {
            self.position += 1;
        }
        current
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        let current = self.advance();
        if current == expected {
            Ok(())
        } else {
            Err(format!("Parser Error: Expected token {:?}, but encountered {:?}", expected, current))
        }
    }

    // --- AST Parsing Rules ---

    pub fn parse_program(&mut self) -> Result<Vec<Statement>, String> {
        let mut program = Vec::new();
        while *self.peek() != Token::EOF {
            program.push(self.parse_statement()?);
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek() {
            Token::Let => self.parse_variable_declaration(),
            Token::Fn => self.parse_function_declaration(),
            Token::Jit => self.parse_jit_directive(),
            Token::Check => self.parse_check_except(),
            other => Err(format!("Parser Error: Unexpected statement starting token {:?}", other)),
        }
    }

    /// Parses: `let user_id: i64 = 42;`
    fn parse_variable_declaration(&mut self) -> Result<Statement, String> {
        self.expect(Token::Let)?;

        let name = match self.advance() {
            Token::Ident(name) => name,
            other => return Err(format!("Expected identifier after 'let', got {:?}", other)),
        };

        self.expect(Token::Colon)?;
        let var_type = self.parse_type()?;

        let initial_value = if *self.peek() == Token::Assign {
            self.advance(); // consume '='
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect(Token::Semicolon)?;

        Ok(Statement::VarDecl {
            name,
            var_type,
            initial_value,
        })
    }

    /// Parses: `fn open_file(path: [u8]) -> !i32 { ... }`
    fn parse_function_declaration(&mut self) -> Result<Statement, String> {
        self.expect(Token::Fn)?;

        let fn_name = match self.advance() {
            Token::Ident(name) => name,
            other => return Err(format!("Expected function name, got {:?}", other)),
        };

        self.expect(Token::LParen)?;
        let mut args = Vec::new();
        while *self.peek() != Token::RParen && *self.peek() != Token::EOF {
            args.push(self.parse_var_decl_argument()?);
            if *self.peek() == Token::Comma {
                self.advance();
            }
        }
        self.expect(Token::RParen)?;

        // Track standard return type details and if the '!' modifier is attached
        let mut return_type = None;
        let mut is_error_contract = false;

        if *self.peek() == Token::Arrow {
            self.advance(); // consume '->'
            if *self.peek() == Token::Exclamation {
                self.advance(); // consume '!'
                is_error_contract = true;
            }
            return_type = Some(self.parse_type()?);
        }

        let body = self.parse_block()?;

        Ok(Statement::FnDecl {
            fn_name,
            args,
            return_type,
            is_error_contract,
            body,
        })
    }

    /// Helper for function argument blocks matching structural signatures
    fn parse_var_decl_argument(&mut self) -> Result<VarDecl, String> {
        let name = match self.advance() {
            Token::Ident(name) => name,
            other => return Err(format!("Expected parameter name, got {:?}", other)),
        };
        self.expect(Token::Colon)?;
        let var_type = self.parse_type()?;
        
        Ok(VarDecl { name, var_type, initial_value: None })
    }

    /// Parses: `jit!("deploy.sh");`
    fn parse_jit_directive(&mut self) -> Result<Statement, String> {
        self.expect(Token::Jit)?;
        self.expect(Token::Exclamation)?;
        self.expect(Token::LParen)?;
        
        let path = match self.advance() {
            Token::StringLiteral(s) => s,
            other => return Err(format!("Expected string path string literal inside jit directive, got {:?}", other)),
        };
        
        self.expect(Token::RParen)?;
        self.expect(Token::Semicolon)?;
        
        Ok(Statement::JitDirective(path))
    }

    /// Parses structural: `check { ... } except { ... }`
    fn parse_check_except(&mut self) -> Result<Statement, String> {
        self.expect(Token::Check)?;
        let check_block = self.parse_block()?;
        
        self.expect(Token::Except)?;
        let except_block = self.parse_block()?;

        Ok(Statement::CheckExcept {
            check_block,
            except_block,
        })
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        self.expect(Token::LBrace)?;
        let mut statements = Vec::new();
        while *self.peek() != Token::RBrace && *self.peek() != Token::EOF {
            statements.push(self.parse_statement()?);
        }
        self.expect(Token::RBrace)?;
        Ok(statements)
    }

    // --- Type Variant Parsing Engine ---

    fn parse_type(&mut self) -> Result<TypeNode, String> {
        match self.advance() {
            Token::Ident(t) => match t.as_str() {
                "bool" => Ok(TypeNode::Bool),
                "i8"   => Ok(TypeNode::I8),
                "i16"  => Ok(TypeNode::I16),
                "i32"  => Ok(TypeNode::I32),
                "i64"  => Ok(TypeNode::I64),
                "i128" => Ok(TypeNode::I128),
                "f32"  => Ok(TypeNode::F32),
                "f64"  => Ok(TypeNode::F64),
                "HashMap" => {
                    self.expect(Token::Ident("less than symbol here".to_string()))?; // Placeholder handling for '<' syntax safely
                    let key = self.parse_type()?;
                    self.expect(Token::Comma)?;
                    let value = self.parse_type()?;
                    // Consume closing '>'
                    self.advance(); 
                    Ok(TypeNode::Map(Box::new(key), Box::new(value)))
                }
                other => Err(format!("Unknown primitive type: {}", other)),
            },
            Token::LBracket => {
                let inner_type = self.parse_type()?;
                if *self.peek() == Token::Semicolon {
                    self.advance(); // consume ';'
                    let size = match self.advance() {
                        Token::IntLiteral(s) => s as u32,
                        other => return Err(format!("Expected array sizing integer, got {:?}", other)),
                    };
                    self.expect(Token::RBracket)?;
                    Ok(TypeNode::Array(Box::new(inner_type), size))
                } else {
                    self.expect(Token::RBracket)?;
                    if let TypeNode::I8 = inner_type {
                        Ok(TypeNode::U8Array) // [u8] optimization mapping
                    } else {
                        Err("Invalid array type configuration formatting".to_string())
                    }
                }
            }
            other => Err(format!("Unexpected token parsed while matching type layout: {:?}", other)),
        }
    }

    // --- Expression Engine (Handles method cascading pipelines & arrays) ---

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_primary_expression()?;

        // Lookahead loop tracking index lookups (`[index]`) and fluent methods (`.then()`)
        loop {
            match self.peek() {
                Token::LBracket => {
                    self.advance(); // consume '['
                    let index = self.parse_expression()?;
                    self.expect(Token::RBracket)?;
                    expr = Expression::IndexAccess {
                        aggregate: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                Token::Dot => {
                    self.advance(); // consume '.'
                    let method = match self.advance() {
                        Token::Ident(name) => name,
                        other => return Err(format!("Expected chain identifier, encountered {:?}", other)),
                    };

                    if method == "then" {
                        self.expect(Token::LParen)?;
                        let next_expr = self.parse_expression()?;
                        self.expect(Token::RParen)?;
                        expr = Expression::Pipeline {
                            lhs: Box::new(expr),
                            rhs: Box::new(next_expr),
                        };
                    } else {
                        return Err(format!("Unsupported pipeline orchestration method: {}", method));
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary_expression(&mut self) -> Result<Expression, String> {
        match self.advance() {
            Token::IntLiteral(val) => Ok(Expression::IntegerLiteral(val)),
            Token::StringLiteral(val) => Ok(Expression::StringLiteral(val)),
            Token::Ident(name) if name == "Process" => {
                self.expect(Token::LParen)?;
                let command = match self.advance() {
                    Token::StringLiteral(cmd) => cmd,
                    other => return Err(format!("Process command target must be a string literal, got {:?}", other)),
                };
                
                let mut args = Vec::new();
                if *self.peek() == Token::Comma {
                    self.advance(); // consume ','
                    self.expect(Token::LBracket)?;
                    while *self.peek() != Token::RBracket && *self.peek() != Token::EOF {
                        args.push(self.parse_expression()?);
                        if *self.peek() == Token::Comma {
                            self.advance();
                        }
                    }
                    self.expect(Token::RBracket)?;
                }
                self.expect(Token::RParen)?;

                Ok(Expression::ProcessCall { command, args })
            }
            other => Err(format!("Failed to parse expected explicit component, encountered token: {:?}", other)),
        }
    }
}
