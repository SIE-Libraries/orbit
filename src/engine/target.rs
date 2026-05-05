use inkwell::targets::{Target, TargetMachine, InitializationConfig, RelocMode, CodeModel};
use inkwell::OptimizationLevel;

pub struct TargetManager {
    pub target_machine: TargetMachine,
}

impl TargetManager {
    pub fn new() -> anyhow::Result<Self> {
        Target::initialize_native(&InitializationConfig::default())
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let target_machine = target
            .create_target_machine(
                &triple,
                "generic",
                "",
                OptimizationLevel::Default,
                RelocMode::Default,
                CodeModel::Default,
            )
            .ok_or_else(|| anyhow::anyhow!("Failed to create target machine"))?;

        Ok(Self { target_machine })
    }
}
