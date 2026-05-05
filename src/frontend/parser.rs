use super::lexer::{Token, TokenInfo};
use super::ast::ASTNode;

pub struct Parser {
    tokens: Vec<TokenInfo>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self { tokens, cursor: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.cursor].token
    }

    fn consume(&mut self) -> &Token {
        let t = &self.tokens[self.cursor].token;
        self.cursor += 1;
        t
    }

    pub fn parse(&mut self) -> Vec<ASTNode> {
        let mut nodes = Vec::new();
        while *self.peek() != Token::EOF {
            nodes.push(self.parse_statement());
        }
        nodes
    }

    fn parse_statement(&mut self) -> ASTNode {
        match self.peek() {
            Token::Let => self.parse_var_decl(),
            _ => {
                self.consume();
                ASTNode::IntegerLiteral(0) // Dummy
            }
        }
    }

    fn parse_var_decl(&mut self) -> ASTNode {
        self.consume(); // let
        let is_mut = if *self.peek() == Token::Mut {
            self.consume();
            true
        } else {
            false
        };

        let name = if let Token::Identifier(id) = self.consume() {
            id.clone()
        } else {
            panic!("Expected identifier");
        };

        self.consume(); // :

        let type_name = match self.consume() {
            Token::TypeI32 => "i32".to_string(),
            Token::TypeI64 => "i64".to_string(),
            Token::TypeU8Array => "[u8]".to_string(),
            _ => "unknown".to_string(),
        };

        let initial_value = if *self.peek() == Token::Assign {
            self.consume();
            Some(Box::new(self.parse_expression()))
        } else {
            None
        };

        self.consume(); // ;

        ASTNode::VarDecl {
            name,
            is_mut,
            type_name,
            initial_value,
        }
    }

    fn parse_expression(&mut self) -> ASTNode {
        match self.consume() {
            Token::LiteralInteger(val) => ASTNode::IntegerLiteral(*val),
            Token::LiteralString(val) => ASTNode::StringLiteral(val.clone()),
            _ => panic!("Unexpected expression"),
        }
    }
}
