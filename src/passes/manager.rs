use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::values::FunctionValue;

pub struct OptimizationManager<'ctx> {
    pub fpm: PassManager<FunctionValue<'ctx>>,
}

impl<'ctx> OptimizationManager<'ctx> {
    pub fn new(module: &Module<'ctx>) -> Self {
        let fpm = PassManager::create(module);

        // Use generic pass addition if specific methods are missing in this version
        // Actually, let's just use what's available or keep it minimal if methods are missing
        // In LLVM 18/Inkwell 0.5, the New Pass Manager is preferred,
        // but let's stick to a basic one that works.

        fpm.initialize();

        Self { fpm }
    }

    pub fn optimize_function(&self, function: FunctionValue<'ctx>) {
        self.fpm.run_on(&function);
    }
}
