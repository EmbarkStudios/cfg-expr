#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

pub mod error;
pub mod expr;
pub mod targets;

pub use error::ParseError;
pub use expr::Expression;
