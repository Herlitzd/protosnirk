use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use parse::verify::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScopeIndex {
    indices: Vec<usize>
}
impl ScopeIndex {
    #[inline]
    pub fn increment(&mut self) {
        trace!("Calling increment on {:?}", self);
        let len = self.indices.len() - 1;
        self.indices[len] += 1;
    }
    #[inline]
    pub fn decrement(&mut self) {
        trace!("Calling decrement on {:?}", self);
        let len = self.indices.len() - 1;
        self.indices[len] -= 1;
    }
    #[inline]
    pub fn push(&mut self) {
        trace!("Calling push on {:?}", self);
        self.indices.push(0);
    }
    #[inline]
    pub fn pop(&mut self) {
        trace!("Calling pop on {:?}", self);
        self.indices.pop();
    }
    pub fn new(vec: Vec<usize>) -> ScopeIndex {
        trace!("Created new scope {:?}", vec);
        ScopeIndex { indices: vec }
    }
}

impl Default for ScopeIndex {
    fn default() -> ScopeIndex {
        trace!("Creating default scope");
        ScopeIndex { indices: vec![0usize] }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SymbolTable {
    values: HashMap<ScopeIndex, Symbol>
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable { values: hashmap![] }
    }
}

impl Deref for SymbolTable {
    type Target = HashMap<ScopeIndex, Symbol>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl DerefMut for SymbolTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

/// Generic structure used by the `SymbolTableChecker` to build the symbol table.
///
/// # Motivation
/// Because of block indentation rules, symbol tables must generally be stored in a tree
/// structure to prevent variables from different blocks from colliding with each other:
/// ```text
/// let x = 0
/// do
///     let y = 0
///     let x = 0
/// let y = 0
/// do
///     let z = 1
///     let y = 0
/// ```
/// There are root blocks, child blocks, and sibiling blocks to differentiate.
///
/// Because Rust's support for defining complex data structures recursively, and updating them
/// (especially with forwards and backwards search) is not very great, we chose a simpler apprach:
/// store the symbols with a special `ScopeIndex`, a list of indices into the scope tree.
/// For example, `z` on line 7 has index `[1, 0]` as it's the first declaration in the second
/// block child of the root.
///
/// In order to build this representation, however, we don't need a tree. Instead, we keep track of
/// this index as we construct the table. We associate a `ScopeIndex` with each encountered
/// variable reference. We only need a stack of `HashMap<String, Symbol>` when going through the
/// AST in order to identify where a variable came from.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct SymbolTableBuilder {
    scopes: Vec<HashMap<String, ScopeIndex>>
}

impl SymbolTableBuilder {
    /// Create a new empty lexical scope manager
    pub fn new() -> SymbolTableBuilder {
        SymbolTableBuilder { scopes: vec![] }
    }

    /// Create a new scope
    pub fn new_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    // Pop the topmost scope from the stack
    pub fn pop(&mut self) -> Option<HashMap<String, ScopeIndex>> {
        self.scopes.pop()
    }

    /// Define a new variable in the local scope
    pub fn define_local(&mut self, name: String, value: ScopeIndex) {
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to define variable {} with no scopes", name);
        let last_ix = self.scopes.len() - 1usize;
        trace!("Defining {} in scope {}", &name, last_ix);
        &mut self.scopes[last_ix].insert(name, value);
    }

    /// Define a variable in the global scope
    pub fn define_global(&mut self, name: String, value: ScopeIndex) {
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to define a global {} with no scopes", name);
        &mut self.scopes[0].insert(name, value);
    }

    /// Get a variable from any scope
    pub fn get(&self, name: &str) -> Option<&ScopeIndex> {
        trace!("Searching for {} in {:#?}", name, self);
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to search for a variable {} with no scopes", name);
        for scope in self.scopes.iter().rev() {
            trace!("Checking for {} in scope {:?}", name, scope);
            if let Some(var_ref) = scope.get(name) {
                return Some(var_ref)
            }
        }
        trace!("Didn't find {}", name);
        None
    }

    /// Get a variable defined in local scopeh
    pub fn get_local(&self, name: &str) -> Option<&ScopeIndex> {
        debug_assert!(!self.scopes.is_empty(),
            "Attempted to get local var {} with no scopes", name);
        let local_scope_ix = self.scopes.len() - 1usize;
        self.scopes[local_scope_ix].get(name)
    }

    /// Get a variable, starting from the given scope
    pub fn get_in_scope(&self, name: &str, scope_level: usize) -> Option<&ScopeIndex> {
        debug_assert!(self.scopes.len() >= scope_level,
            "Do not have {} scopes to search, only have {}", scope_level, self.scopes.len());
        for scope in self.scopes[0..scope_level].iter().rev() {
            if let Some(var_ref) = scope.get(name) {
                return Some(var_ref)
            }
        }
        None
    }
}
