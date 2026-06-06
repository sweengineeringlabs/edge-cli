//! Public API layer — traits and types.
//!
//! Theme dirs (`main/`, `provider/`, `scaffold/`, `validator/`) carry the
//! inner SEA layout; crate-level `traits/`, `types/`, `error/`, and `vo/`
//! hold cross-theme contracts consumed by two or more themes.

pub mod error;
pub mod main;
pub mod provider;
pub mod scaffold;
pub mod types;
pub mod validator;
pub mod vo;
