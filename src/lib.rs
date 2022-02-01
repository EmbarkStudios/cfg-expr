// BEGIN - Embark standard lints v5 for Rust 1.55+
// do not change or add/remove here, but one can add exceptions after this section
// for more info see: <https://github.com/EmbarkStudios/rust-ecosystem/issues/59>
#![deny(unsafe_code)]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::disallowed_method,
    clippy::disallowed_type,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::flat_map_option,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::from_iter_instead_of_collect,
    clippy::if_let_mutex,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wild_err_arm,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::missing_enforced_import_renames,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::rc_mutex,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::semicolon_if_nothing_returned,
    clippy::single_match_else,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
// END - Embark standard lints v0.5 for Rust 1.55+
// crate-specific exceptions:
#![allow(clippy::single_match_else)]

//! cfg-expr is a crate that can be used to parse and evaluate Rust `cfg()`
//! expressions, both as declarable in Rust code itself, as well in cargo
//! manifests' `[target.'cfg()'.dependencies]` sections.
//!
//! It contains a list of all builtin targets known to rustc as of `1.54.0` that
//! can be used to determine if a particular cfg expression is satisfiable.
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
//! // cfg_expr includes a list of every builtin target in rustc
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
