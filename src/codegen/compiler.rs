use inkwell::values::PointerValue;
use inkwell::types::BasicType;
use crate::frontend::ast::ASTNode;
use super::context::CodeGenContext;
use std::collections::HashMap;

pub struct Compiler<'a, 'ctx> {
    pub cg_context: &'a CodeGenContext<'ctx>,
    pub variables: HashMap<String, PointerValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(cg_context: &'a CodeGenContext<'ctx>) -> Self {
        Self {
            cg_context,
            variables: HashMap::new(),
        }
    }

    pub fn compile_module(&mut self, nodes: &[ASTNode]) -> anyhow::Result<()> {
        for node in nodes {
            self.compile_node(node)?;
        }
        Ok(())
    }

    fn compile_node(&mut self, node: &ASTNode) -> anyhow::Result<()> {
        match node {
            ASTNode::VarDecl { name, is_mut: _, type_name, initial_value } => {
                let ty = match type_name.as_str() {
                    "i32" => self.cg_context.context.i32_type().as_basic_type_enum(),
                    "i64" => self.cg_context.context.i64_type().as_basic_type_enum(),
                    _ => anyhow::bail!("Unsupported type"),
                };
                let alloca = self.cg_context.builder.build_alloca(ty, name)?;
                self.variables.insert(name.clone(), alloca);
                if let Some(init) = initial_value {
                    if let ASTNode::IntegerLiteral(val) = **init {
                        let const_val = self.cg_context.context.i64_type().const_int(val as u64, false);
                        self.cg_context.builder.build_store(alloca, const_val)?;
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
