use rustpython_ast::Keyword;

use crate::ast::types::Range;
use crate::flake8_bandit::helpers::{matches_password_name, string_literal};
use crate::registry::{Check, CheckKind};
use crate::violations;

/// S106
pub fn hardcoded_password_func_arg(keywords: &[Keyword]) -> Vec<Check> {
    keywords
        .iter()
        .filter_map(|keyword| {
            let string = string_literal(&keyword.node.value)?;
            let arg = keyword.node.arg.as_ref()?;
            if !matches_password_name(arg) {
                return None;
            }
            Some(Check::new(
                violations::HardcodedPasswordFuncArg(string.to_string()),
                Range::from_located(keyword),
            ))
        })
        .collect()
}
