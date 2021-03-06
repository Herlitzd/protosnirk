//! Definition of types in protosnirk

use std::collections::HashMap;

/// Representation of types in protosnirk
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    /// `()`
    Empty,
    /// Standard type for now
    Float,
    /// Function - only used in declarations
    Fn(FnType)
}
impl Type {
    pub fn expect_fn(self) -> FnType {
        match self {
            Type::Fn(inner) => inner,
            other => panic!("`expect_fn` called on {:?}", other)
        }
    }
}

/// Type representation of functions in protosnirk
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnType {
    return_type: Box<Type>,
    // Order must be preserved
    args: Vec<(String, Type)>
}
impl FnType {
    pub fn new(return_type: Box<Type>, args: Vec<(String, Type)>) -> FnType {
        FnType { return_type: return_type, args: args }
    }
    pub fn get_return(&self) -> &Type {
        &self.return_type
    }
    pub fn get_args(&self) -> &[(String, Type)] {
        &self.args
    }
    pub fn get_arg(&self, name: &str) -> Option<(usize, Type)> {
        for (ix, arg) in self.args.iter().enumerate() {
            if arg.0 == name {
                return Some((ix, arg.1.clone()))
            }
        }
        return None
    }
}
