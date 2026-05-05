pub enum ASTNode {
    VarDecl {
        name: String,
        is_mut: bool,
        type_name: String,
        initial_value: Option<Box<ASTNode>>,
    },
    FnDecl {
        name: String,
        args: Vec<ASTNode>, // List of VarDecl
        return_type: Option<String>,
        body: Vec<ASTNode>,
    },
    IntegerLiteral(i64),
    StringLiteral(String),
    ProcessCall {
        command: String,
        args: Vec<ASTNode>,
    },
    Pipeline {
        nodes: Vec<ASTNode>,
    },
    CheckExcept {
        check_block: Vec<ASTNode>,
        except_block: Vec<ASTNode>,
    },
    JitDirective(String),
}
