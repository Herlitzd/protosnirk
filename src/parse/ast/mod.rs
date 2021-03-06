mod expression;
mod item;
mod stmt;
mod operator;

pub use self::expression::*;
pub use self::item::*;
pub use self::stmt::*;
pub use self::operator::Operator;

use std::cell::RefCell;

use lex::Token;
use parse::verify::scope::ScopeIndex;

/// Basic identifier type
#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub token: Token,
    pub index: RefCell<ScopeIndex>
}
impl Identifier {
    pub fn new(token: Token) -> Self {
        Identifier { token: token, index: RefCell::new(ScopeIndex::default()) }
    }
    pub fn get_name(&self) -> &str {
        &self.token.text
    }
    pub fn get_token(&self) -> &Token {
        &self.token
    }

    pub fn get_index(&self) -> ScopeIndex {
        self.index.borrow().clone()
    }

    pub fn set_index(&self, index: ScopeIndex) {
        *self.index.borrow_mut() = index;
    }
}
impl Into<Token> for Identifier {
    fn into(self) -> Token {
        self.token
    }
}

/// Collection of statements which may have an expression value
#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub partial_index: ScopeIndex
}
impl Block {
    pub fn new(statements: Vec<Statement>) -> Block {
        Block { statements: statements, partial_index: ScopeIndex::default() }
    }
    pub fn has_value(&self) -> bool {
        if self.statements.len() == 0 {
            return false
        }
        let last_ix = self.statements.len() - 1;
        // TODO actual analysis
        for (ix, statement) in self.statements.iter().enumerate() {
            if ix == last_ix {
                return statement.has_value()
            }
            // else if stmt == return {
            //     return stmt.has_value()
            // }
        }
        return false
    }
    pub fn get_index(&self) -> &ScopeIndex {
        &self.partial_index
    }
    pub fn get_stmts(&self) -> &[Statement] {
        &self.statements
    }
}
