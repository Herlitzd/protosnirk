//! ISO date parse example

#![allow(unused_imports, dead_code)]

use std::str;
use std::borrow::ToOwned;
use nom::{self, IResult};

// A variable binding
#[derive(PartialEq, Eq, Debug)]
pub struct Binding {
    pub name: String,
    pub mutable: bool,
    pub value: String
}

// Example
// Parse `let x = 0` into Binding { name: 'x', mutable: false, val: 0 }

macro_rules! keyword {
    ($kw:ident) => {
        named!(concat!(keyword_, $kw) <&[u8], &[u8]>, tag!(stringify!($kw)));
    };
    (pub $kw:ident) => {
        named!(pub concat!(keyword_, $kw) <&[u8], &[u8]>, tag!(stringify!($kw)));
    };

    ($name:ident = $kw:expr) => {
        named!($name <&[u8], &[u8]>, tag!($kw));
    };
    (pub $name:ident = $kw:expr) => {
        named!(pub $name <&[u8], &[u8]>, tag!($kw));
    };
}

keyword!(pub keyword_let = "let");
keyword!(pub keyword_eq = "=");
keyword!(pub keyword_mut = "mut");

named!(pub get_digits <&[u8], &[u8]>,
       take_while1!(
           nom::is_digit
        ));

named!(pub spacing <&[u8], &[u8]>,
       take_while1!(
           nom::is_space
        ));

named!(pub ident <&[u8], &[u8]>,
       take_while!(
           nom::is_alphanumeric
       ));

named!(pub declaration <&[u8], bool>,
       alt!(
           keyword_let => { |_| false } |
           keyword_mut => { |_| true }
       ));

fn ident_start(input: u8) -> bool {
    input == b'_' || nom::is_alphanumeric(input)
}

/*
fn identifier(input: &[u8]) -> IResult<&[u8], &[u8]> {
    if input.len() == 0 {
        return IResult::Incomplete
    }
    if !ident_start(input[0]) {
        return IResult::Error(&b"err"[..])
    }
    for ch in input[1..] {
        if !nom::is_alphanumeric(ch) {
            return IResult::Error(2)
        }
    }
    IResult::Done(&[], input)
}
 */

named!(pub binding <&[u8], Binding>,
       chain!(
           decl: declaration ~
           spacing           ~
           name: ident       ~
           spacing           ~
           keyword_eq        ~
           spacing           ~
           val_str: get_digits,
           || {
               Binding {
                   name: get_string(name),
                   mutable: decl,
                   value: get_string(val_str)
               }
           }
       ));

fn get_string(input: &[u8]) -> String {
    str::from_utf8(input).unwrap().to_string()
}

fn not_keyword(input: &[u8]) -> bool {
    const KEYWORDS: &'static[&'static[u8]] = &[b"let", b"mut"];

    for word in KEYWORDS {
        if input == *word {
            return true
        }
    }
    return false
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;
    use nom::IResult::*;

    const EMPTY: &'static [u8] = &[];

    /// assert_parse!(parser <- input, Value);
    macro_rules! assert_parse {
        ($parser:ident <- $input:expr, $value:expr) => {
            assert_eq!($parser(&$input[..]), ::nom::IResult::Done(EMPTY, $value));
        };
    }

    #[test]
    fn binding_works() {
        assert_eq!(binding(&b"let foo = 12"[..]),
                           Done(EMPTY, Binding { name: "foo".to_string(), mutable: false, value: "12".to_string() }) );
        assert_parse!(binding <-
                      b"mut foobarbaz = 1234",
                      Binding {
                          name: "foobarbaz".into(),
                          mutable: true,
                          value: "1234".into()
                      });
        assert_parse!(
            binding <- b"let asdf = 22243",
            Binding {
                name: "asdf".into(),
                mutable: false,
                value: "22243".into()
            });
    }
}
