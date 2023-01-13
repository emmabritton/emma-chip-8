use clap_common::CheckLevel;
use crate::ast::Program;

impl Program {
    pub fn warnings(&self, ec8_level: CheckLevel, lint_level: CheckLevel) -> Option<String> {
        None
    }
}
