#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

//! cfg-expr is a crate that can be used to parse and evaluate Rust cfg() expressions,
//! both as declarable in Rust code itself, as well in cargo's `[target.'cfg()'.dependencies]` sections.
//!
//! It contains a list of all builtin targets known to rustc as of 1.41 that can be used
//! to determine if a particular cfg expression is satisfiable.
//!
//! ```
//! use cfg_expr::{targets::get_builtin_target_by_triple, Expression, Predicate};
//!
//! let specific = Expression::parse(
//!     r#"all(
//!         target_os = "windows",
//!         target_arch = "x86",
//!         windows,
//!         target_env = "msvc",
//!         target_feature = "fxsr",
//!         target_feature = "sse",
//!         target_feature = "sse2",
//!         target_pointer_width = "32",
//!         target_endian = "little",
//!         not(target_vendor = "uwp"),
//!         feature = "cool_thing",
//!     )"#,
//! )
//! .unwrap();
//!
//! // cfg_expr includes a list of every builtin target in rustc (as of 1.41)
//! let x86_win = get_builtin_target_by_triple("i686-pc-windows-msvc").unwrap();
//! let x86_pentium_win = get_builtin_target_by_triple("i586-pc-windows-msvc").unwrap();
//! let uwp_win = get_builtin_target_by_triple("i686-uwp-windows-msvc").unwrap();
//! let mac = get_builtin_target_by_triple("x86_64-apple-darwin").unwrap();
//!
//! let avail_targ_feats = ["fxsr", "sse", "sse2"];
//!
//! // This will satisfy all requirements
//! assert!(specific.eval(|pred| {
//!     match pred {
//!         Predicate::Target(tp) => tp.matches(x86_win),
//!         Predicate::TargetFeature(feat) => avail_targ_feats.contains(feat),
//!         Predicate::Feature(feat) => *feat == "cool_thing",
//!         _ => false,
//!     }
//! }));
//!
//! // This won't, it doesnt' have the cool_thing feature!
//! assert!(!specific.eval(|pred| {
//!     match pred {
//!         Predicate::Target(tp) => tp.matches(x86_pentium_win),
//!         Predicate::TargetFeature(feat) => avail_targ_feats.contains(feat),
//!         _ => false,
//!     }
//! }));
//!
//! // This will *not* satisfy the vendor predicate
//! assert!(!specific.eval(|pred| {
//!     match pred {
//!         Predicate::Target(tp) => tp.matches(uwp_win),
//!         Predicate::TargetFeature(feat) => avail_targ_feats.contains(feat),
//!         _ => false,
//!     }
//! }));
//!
//! // This will *not* satisfy the vendor, os, or env predicates
//! assert!(!specific.eval(|pred| {
//!     match pred {
//!         Predicate::Target(tp) => tp.matches(mac),
//!         Predicate::TargetFeature(feat) => avail_targ_feats.contains(feat),
//!         _ => false,
//!     }
//! }));
//! ```

/// Types related to parse errors
pub mod error;
/// Types related to cfg expressions
pub mod expr;
/// Types related to rustc targets
pub mod targets;

pub use error::ParseError;
pub use expr::{Expression, Predicate, TargetPredicate};

#[cfg(feature = "targets")]
pub use target_lexicon;
