//! Parser module
//!
//! The parser is responsible for turning the incoming token stream into
//! an AST. It emits errors if the code is written incorrectly (i.e. missing
//! paren, as well as some linting errors such as spacing that occur at the token
//! level.
//!
//! The parser will pass an AST (`Program`) to the checker, which will verify
//! that it is a leagal protosnirk program.

mod scoped_id;
mod errors;
mod parser;

pub mod ast;
pub mod symbol;

#[cfg(test)]
pub mod tests;

pub use self::errors::{ParseError, ParseResult, ExpectedNextType};
pub use self::parser::{Parser, IndentationRule};
pub use self::scoped_id::ScopedId;
