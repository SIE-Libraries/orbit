use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;

pub struct CodeGenContext<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}

impl<'ctx> CodeGenContext<'ctx> {
    pub fn new(context: &'ctx Context, name: &str) -> Self {
        let module = context.create_module(name);
        let builder = context.create_builder();
        Self {
            context,
            module,
            builder,
        }
    }
}
