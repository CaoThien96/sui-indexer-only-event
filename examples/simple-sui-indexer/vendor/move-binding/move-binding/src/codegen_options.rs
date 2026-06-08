use move_core_types::account_address::AccountAddress;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EmitMode {
    #[default]
    All,
    EventStructs,
    ModuleStructs,
    Indexed,
}

#[derive(Clone, Debug, Default)]
pub struct CodegenOptions {
    pub register_only: bool,
    pub emit_mode: EmitMode,
    pub modules: Option<Vec<String>>,
    pub event_modules: Option<Vec<String>>,
    pub support_modules: Option<Vec<String>>,
    /// `address=alias` pairs for cross-package type resolution at codegen time.
    pub linkage: HashMap<AccountAddress, String>,
}

impl CodegenOptions {
    pub fn should_emit_module(&self, module_name: &str) -> bool {
        if self.register_only {
            return false;
        }
        match (&self.modules, &self.event_modules, &self.support_modules) {
            (Some(modules), _, _) => modules.iter().any(|m| m == module_name),
            (_, Some(event), Some(support)) => {
                event.iter().any(|m| m == module_name)
                    || support.iter().any(|m| m == module_name)
            }
            (_, Some(event), None) => event.iter().any(|m| m == module_name),
            (None, None, None) => true,
            _ => false,
        }
    }

    pub fn module_emit_mode(&self, module_name: &str) -> EmitMode {
        if let Some(support) = &self.support_modules {
            if support.iter().any(|m| m == module_name) {
                return EmitMode::ModuleStructs;
            }
        }
        if let Some(events) = &self.event_modules {
            if events.iter().any(|m| m == module_name) {
                return EmitMode::EventStructs;
            }
        }
        self.emit_mode
    }
}
