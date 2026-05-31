use inkwell::context::Context;

use orbit::ast::{Expression, Statement, TypeNode};
use orbit::compiler::Compiler;

fn main() {
    let context = Context::create();
    let mut compiler = Compiler::new(&context);

    let program = vec![Statement::FnDecl {
        fn_name: "main".to_string(),
        args: vec![],
        return_type: None,
        is_error_contract: false,
        body: vec![Statement::VarDecl {
            name: "answer".to_string(),
            var_type: TypeNode::I32,
            initial_value: Some(Expression::IntegerLiteral(42)),
        }],
    }];

    compiler
        .compile(&program)
        .expect("Failed to compile program");
    println!("{}", compiler.print_ir());
}
