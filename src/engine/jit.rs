use inkwell::OptimizationLevel;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;

pub struct JitEngine<'ctx> {
    pub execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> JitEngine<'ctx> {
    pub fn new(module: Module<'ctx>) -> anyhow::Result<Self> {
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::Default)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        Ok(Self { execution_engine })
    }
}
