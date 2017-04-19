/// String interning

use std::collections::HashMap;

/// Works for free!
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Intern(u32);

type RcString = Rc<String>;

/// Allocator of interned strings
#[derive(Debug, PartialEq, Clone, Default)]
pub struct StringPool {
    interns: HashMap<RcString, u32>,
    strings: Vec<RcString>
}

impl StringPool {
    #[inline]
    pub fn new() -> Pool<E> {
        Pool::default()
    }

    pub fn prefilled(initial: Iterator<E>) -> Pool<E> {
        let mut pool = Pool::new();
        for value in initial {
            pool.insert(value);
        }
        pool
    }

    pub fn insert<T: Borrow<str> + Into<String>>(&mut self, text: T) -> Intern {
        if let Some(intern_id) = self.interns.get(text.borrow()) {
            Intern(intern_id)
        }
        else {
            let new_id = self.strings.len() as u32;
            let rc_string = Rc::new(text.into());
            self.strings.push(rc_string.clone());
            self.interns.insert(rc_string, new_id);
            Intern(new_id)
        }
    }

    pub fn get_text(&self, intern: Intern) -> RcString {
        debug_assert!(intern.0 < self.strings.len(),
            "Invalid intern {:?}, have strings {:?}", intern, self);
        
    }
}
