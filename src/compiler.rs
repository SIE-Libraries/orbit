use anyhow::Result;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::PointerValue;
use std::collections::HashMap;

use crate::ast::{Statement, TypeNode};

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub named_values: HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("orbit_jit");
        let builder = context.create_builder();

        Self {
            context,
            module,
            builder,
            named_values: HashMap::new(),
        }
    }

    pub fn compile(&mut self, statements: &[Statement]) -> Result<()> {
        for statement in statements {
            statement.codegen(self);
        }
        Ok(())
    }

    pub fn type_to_llvm(&self, ty: &TypeNode) -> Option<BasicTypeEnum<'ctx>> {
        match ty {
            TypeNode::Bool => Some(self.context.bool_type().into()),
            TypeNode::I8 => Some(self.context.i8_type().into()),
            TypeNode::I16 => Some(self.context.i16_type().into()),
            TypeNode::I32 => Some(self.context.i32_type().into()),
            TypeNode::I64 => Some(self.context.i64_type().into()),
            TypeNode::I128 => Some(self.context.i128_type().into()),
            TypeNode::F32 => Some(self.context.f32_type().into()),
            TypeNode::F64 => Some(self.context.f64_type().into()),
            TypeNode::U8Array => Some(
                self.context
                    .i8_type()
                    .ptr_type(inkwell::AddressSpace::from(0))
                    .into(),
            ),
            TypeNode::Array(element_type, size) => self
                .type_to_llvm(element_type)
                .map(|elem| elem.array_type(*size).into()),
            TypeNode::Map(_, _) => None,
        }
    }

    pub fn create_entry_block_alloca(
        &self,
        function: inkwell::values::FunctionValue<'ctx>,
        name: &str,
        ty: BasicTypeEnum<'ctx>,
    ) -> PointerValue<'ctx> {
        let builder = self.context.create_builder();
        let entry = function.get_first_basic_block().unwrap();
        builder.position_at_end(entry);
        builder.build_alloca(ty, name)
    }

    pub fn print_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }
}
