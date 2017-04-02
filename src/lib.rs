#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate maplit; // Convenience for initializing parser for now
extern crate unicode_categories; // Used by tokenizer for valid idents
extern crate smallvec; // Optimize storage in ScopedIds

extern crate iron_llvm;
extern crate llvm_sys;

pub mod lex;
pub mod parse;
pub mod check;
pub mod compile;
pub mod run;

#[cfg(test)]
mod tests;
