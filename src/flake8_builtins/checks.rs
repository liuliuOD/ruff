use rustpython_ast::Located;

use crate::ast::types::Range;
use crate::flake8_builtins::types::ShadowingType;
use crate::python::builtins::BUILTINS;
use crate::registry::{Check, CheckKind};
use crate::violations;

/// Check builtin name shadowing.
pub fn builtin_shadowing<T>(
    name: &str,
    located: &Located<T>,
    node_type: ShadowingType,
) -> Option<Check> {
    if BUILTINS.contains(&name) {
        Some(Check::new(
            match node_type {
                ShadowingType::Variable => violations::BuiltinVariableShadowing(name.to_string()),
                ShadowingType::Argument => violations::BuiltinArgumentShadowing(name.to_string()),
                ShadowingType::Attribute => violations::BuiltinAttributeShadowing(name.to_string()),
            },
            Range::from_located(located),
        ))
    } else {
        None
    }
}
