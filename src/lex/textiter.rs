//! Trait for iterating over text

use lex::TextLocation;
use std::iter::{Iterator, Peekable};

/// A specialized iterator (for tokenizing) which also implements `peek()`
/// and keeps track of its location.
pub trait TextIter : Iterator {
    fn peek(&mut self) -> Option<char>;
    fn get_location(&self) -> TextLocation;
}

/// A `TextIter` which uses an internal `Peekable<T>`.
#[derive(Debug, Clone)]
pub struct PeekTextIter<T> where T: Iterator<Item=char> {
    /// Iterator which does most of the work
    iter: Peekable<T>,
    /// Current line in the source
    current_line: usize,
    /// Current column in the source
    current_column: usize,
    /// Current byte in the source
    current_char: usize
}
impl<T: Iterator<Item=char>> PeekTextIter<T> {
    pub fn new(iter: Peekable<T>) -> PeekTextIter<T> {
        PeekTextIter {
            iter: iter,
            current_line: 0,
            current_column: 0,
            current_char: 0
        }
    }
}

impl<T: Iterator<Item=char>> TextIter for PeekTextIter<T> {
    fn peek(&mut self) -> Option<char> {
        self.iter.peek().cloned()
    }
    fn get_location(&self) -> TextLocation {
        TextLocation {
            start_char: self.current_char,
            start_line: self.current_line,
            start_column: self.current_column
        }
    }
}

impl<T: Iterator<Item=char>> Iterator for PeekTextIter<T> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let result = self.iter.next();
        self.current_char = self.current_char.saturating_add(1);
        match result {
            Some('\n') => {
                self.current_line = self.current_line.saturating_add(1);
                self.current_column = 0;
            },
            Some(_) => {
                self.current_column = self.current_column.saturating_add(1);
            },
            None => {}
        }
        result
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_starts_at_zero() {
        //let empty_peek = "".into().into_iter().peekable();
        //let empty_textiter = PeekTextIter::new(empty_peek);
    }
}
