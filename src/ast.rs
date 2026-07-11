use crate::compiler::Compiler;
use inkwell::values::BasicValueEnum;
use inkwell::AddressSpace;

#[derive(Clone, Debug)]
pub enum TypeNode {
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    U8Array,
    Array(Box<TypeNode>, u32),
    Map(Box<TypeNode>, Box<TypeNode>),
}

#[derive(Clone, Debug)]
pub struct VarDecl {
    pub name: String,
    pub var_type: TypeNode,
    pub initial_value: Option<Expression>,
}

#[derive(Clone, Debug)]
pub enum Expression {
    IntegerLiteral(i64),
    StringLiteral(String),
    ProcessCall {
        command: String,
        args: Vec<Expression>,
    },
    Pipeline {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    IndexAccess {
        aggregate: Box<Expression>,
        index: Box<Expression>,
    },
}

#[derive(Clone, Debug)]
pub enum Statement {
    VarDecl {
        name: String,
        var_type: TypeNode,
        initial_value: Option<Expression>,
    },
    FnDecl {
        fn_name: String,
        args: Vec<VarDecl>,
        return_type: Option<TypeNode>,
        is_error_contract: bool,
        body: Vec<Statement>,
    },
    JitDirective(String),
    CheckExcept {
        check_block: Vec<Statement>,
        except_block: Vec<Statement>,
    },
}

impl Expression {
    pub fn codegen<'ctx>(&self, compiler: &mut Compiler<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        match self {
            Expression::IntegerLiteral(value) => Some(
                compiler
                    .context
                    .i64_type()
                    .const_int(*value as u64, true)
                    .into(),
            ),
            Expression::StringLiteral(text) => Some(
                compiler
                    .builder
                    .build_global_string_ptr(text, "str_literal")
                    .as_pointer_value()
                    .into(),
            ),
            Expression::ProcessCall { command, args: _args } => {
                // TODO: Implement args processing when building full process call support
                // For now, we're generating the basic function signature
                let i32_type = compiler.context.i32_type();
                let i8_ptr_type = compiler
                    .context
                    .i8_type()
                    .ptr_type(AddressSpace::from(0));
                let char_ptr_ptr_type = i8_ptr_type.ptr_type(AddressSpace::from(0));

                let func_type = i32_type.fn_type(
                    &[i8_ptr_type.into(), char_ptr_ptr_type.into()],
                    false,
                );

                let spaceship_fn = compiler
                    .module
                    .get_function("spaceship_run_process")
                    .unwrap_or_else(|| {
                        compiler
                            .module
                            .add_function("spaceship_run_process", func_type, None)
                    });

                let command_ptr = compiler
                    .builder
                    .build_global_string_ptr(command, "command")
                    .as_pointer_value();

                let args_ptr = char_ptr_ptr_type.const_null();

                let call_site = compiler.builder.build_call(
                    spaceship_fn,
                    &[command_ptr.into(), args_ptr.into()],
                    "run_process",
                );

                call_site.try_as_basic_value().left()
            }
            Expression::Pipeline { .. } => None,
            Expression::IndexAccess { .. } => None,
        }
    }
}

impl Statement {
    pub fn codegen<'ctx>(&self, compiler: &mut Compiler<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        match self {
            Statement::VarDecl {
                name,
                var_type,
                initial_value,
            } => {
                let llvm_type = compiler.type_to_llvm(var_type)?;
                let current_function = compiler.builder.get_insert_block()?.get_parent()?;
                let entry_block = current_function.get_first_basic_block()?;

                let entry_builder = compiler.context.create_builder();
                entry_builder.position_at_end(entry_block);
                let variable = entry_builder.build_alloca(llvm_type, name);

                if let Some(expr) = initial_value {
                    let initial_value = expr.codegen(compiler)?;
                    compiler.builder.build_store(variable, initial_value);
                }

                compiler.named_values.insert(name.clone(), variable);
                Some(variable.into())
            }
            Statement::FnDecl {
                fn_name,
                args,
                return_type,
                body,
                ..
            } => {
                let arg_types = args
                    .iter()
                    .map(|arg| compiler.type_to_llvm(&arg.var_type))
                    .collect::<Option<Vec<_>>>()?;

                let metadata_args: Vec<_> = arg_types.iter().map(|t| (*t).into()).collect();

                let function_type = if let Some(return_type) = return_type {
                    match compiler.type_to_llvm(return_type)? {
                        inkwell::types::BasicTypeEnum::IntType(int_type) => {
                            int_type.fn_type(&metadata_args, false)
                        }
                        inkwell::types::BasicTypeEnum::FloatType(float_type) => {
                            float_type.fn_type(&metadata_args, false)
                        }
                        inkwell::types::BasicTypeEnum::PointerType(ptr_type) => {
                            ptr_type.fn_type(&metadata_args, false)
                        }
                        _ => return None,
                    }
                } else {
                    compiler.context.void_type().fn_type(&metadata_args, false)
                };

                let function = compiler.module.add_function(fn_name, function_type, None);
                let entry = compiler.context.append_basic_block(function, "entry");
                compiler.builder.position_at_end(entry);

                compiler.named_values.clear();
                for (index, arg_value) in function.get_param_iter().enumerate() {
                    let arg_name = args[index].name.clone();
                    arg_value.set_name(&arg_name);
                    let alloca = compiler.create_entry_block_alloca(
                        function,
                        &arg_name,
                        arg_value.get_type().into(),
                    );
                    compiler.builder.build_store(alloca, arg_value);
                    compiler.named_values.insert(arg_name, alloca);
                }

                for statement in body {
                    statement.codegen(compiler);
                }

                if function.get_type().get_return_type().is_none() {
                    compiler.builder.build_return(None);
                } else {
                    let zero_value: inkwell::values::BasicValueEnum<'ctx> =
                        compiler.context.i32_type().const_zero().into();
                    compiler.builder.build_return(Some(&zero_value));
                }

                if function.verify(true) {
                    Some(function.as_global_value().as_pointer_value().into())
                } else {
                    None
                }
            }
            Statement::JitDirective(_) => None,
            Statement::CheckExcept { .. } => None,
        }
    }
}
