//! Mid-sized integration tests for protosnirk.
//! Most testing is done in the external tests.

use std::borrow::Cow;
use std::str::Chars;

use lex::{Token, TokenType, TokenData, TextLocation, Tokenizer, IterTokenizer};

macro_rules! match_tokens {
    ($tokenizer:ident { $($token:expr),* }) => {
        $(
            let next = $tokenizer.next();
            let expected = $token;
            assert!(next == expected,
                "\nExpected: {:#?}\nActual: {:#?}", expected, next);
        )*
    }
}

fn make_tokenizer<'a>(input: &'a str) -> IterTokenizer<Chars<'a>> {
    IterTokenizer::new(input.chars())
}

#[test]
fn it_grabs_single_keyword() {
    let input = "let";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Keyword,
            text: Cow::Borrowed("let"),
            location: TextLocation {
                start_char: 0,
                start_line: 0,
                start_column: 0
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 3,
                start_line: 0,
                start_column: 3
            }
        }
    });
    let input = "mut";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Keyword,
            text: Cow::Borrowed("mut"),
            location: TextLocation {
                start_char: 0,
                start_line: 0,
                start_column: 0
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 3,
                start_line: 0,
                start_column: 3
            }
        }
    });
    let input = "return";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Keyword,
            text: Cow::Borrowed("return"),
            location: TextLocation {
                start_char: 0,
                start_line: 0,
                start_column: 0
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 6,
                start_line: 0,
                start_column: 6
            }
        }
    });
}

#[test]
fn it_grabs_prefix_symbol_at_end_of_file() {
    let input = "+";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("+"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 1,
                start_line: 0,
                start_column: 1
            }
        }
    });
}
#[test]
fn it_grabs_adjacent_prefix_symbols() {
    let input = "+-";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("+"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("-"),
            location: TextLocation {
                start_char: 1,
                start_line: 0,
                start_column: 1
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 2,
                start_line: 0,
                start_column: 2
            }
        }
    });
}

#[test]
fn it_grabs_prefix_symbol_mid_file() {
    let input = "+ ";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("+"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 2,
                start_line: 0,
                start_column: 2
            }
        }
    });
}


#[test]
fn it_gabs_unmatching_parens() {
    let input = "((";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("("),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("("),
            location: TextLocation {
                start_char: 1,
                start_line: 0,
                start_column: 1
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 2,
                start_line: 0,
                start_column: 2
            }
        }
    });
}

#[test]
fn it_grabs_matching_parens() {
    let input = "()";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("("),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed(")"),
            location: TextLocation {
                start_char: 1,
                start_line: 0,
                start_column: 1
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 2,
                start_line: 0,
                start_column: 2
            }
        }
    });
}

#[test]
fn it_grabs_single_ident() {
    let input = "anIdentifier_2";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("anIdentifier_2"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 14,
                start_line: 0,
                start_column: 14
            }
        }
    });
}
#[test]
fn it_grabs_let_ident() {
    let input = "let x";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Keyword,
            text: Cow::Borrowed("let"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("x"),
            location: TextLocation {
                start_char: 4,
                start_line: 0,
                start_column: 4
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 5,
                start_line: 0,
                start_column: 5
            }
        }
    });
}

#[test]
fn it_grabs_float_literal() {
    let input = "224";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::NumberLiteral(224f64),
            text: Cow::Borrowed("224"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 3,
                start_line: 0,
                start_column: 3
            }
        }
    });
    let input = "2.4";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::NumberLiteral(2.4f64),
            text: Cow::Borrowed("2.4"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 3,
                start_line: 0,
                start_column: 3
            }
        }
    });
    let input = "2e4";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::NumberLiteral(2e4f64),
            text: Cow::Borrowed("2e4"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 3,
                start_line: 0,
                start_column: 3
            }
        }
    });
    let input = "2.4e4";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::NumberLiteral(2.4e4f64),
            text: Cow::Borrowed("2.4e4"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 5,
                start_line: 0,
                start_column: 5
            }
        }
    });
}

#[test]
fn it_ignores_whitespace() {
    let input = "\n\t\r\n";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 4,
                start_line: 2,
                start_column: 0
            }
        }
    });
}

#[test]
fn it_ignores_line_comment() {
    let input =
    "//comment\nlet x";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Keyword,
            text: Cow::Borrowed("let"),
            location: TextLocation {
                start_char: 10,
                start_line: 1,
                start_column: 0
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("x"),
            location: TextLocation {
                start_char: 14,
                start_line: 1,
                start_column: 4
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 15,
                start_line: 1,
                start_column: 5
            }
        }
    });
}

#[test]
fn it_lexes_complex_input() {
    let input =
    "let x = y \
     y += 55e7\t \n\
     return y % x + 224.5".into();
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Keyword,
            text: Cow::Borrowed("let"),
            location: TextLocation {
                start_char: 0,
                start_line: 0,
                start_column: 0
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("x"),
            location: TextLocation {
                start_char: 4,
                start_line: 0,
                start_column: 4
            }
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("="),
            location: TextLocation {
                start_char: 6,
                start_line: 0,
                start_column: 6
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("y"),
            location: TextLocation {
                start_char: 8,
                start_line: 0,
                start_column: 8
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("y"),
            location: TextLocation {
                start_char: 10,
                start_line: 0,
                start_column: 10
            }
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("+="),
            location: TextLocation {
                start_char: 12,
                start_line: 0,
                start_column: 12
            }
        },
        Token {
            data: TokenData::NumberLiteral(55e7f64),
            text: Cow::Borrowed("55e7"),
            location: TextLocation {
                start_char: 15,
                start_line: 0,
                start_column: 15
            }
        },
        Token {
            data: TokenData::Keyword,
            text: Cow::Borrowed("return"),
            location: TextLocation {
                start_char: 22,
                start_line: 1,
                start_column: 0
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("y"),
            location: TextLocation {
                start_char: 29,
                start_line: 1,
                start_column: 7
            }
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("%"),
            location: TextLocation {
                start_char: 31,
                start_line: 1,
                start_column: 9
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("x"),
            location: TextLocation {
                start_char: 33,
                start_line: 1,
                start_column: 11
            }
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("+"),
            location: TextLocation {
                start_char: 35,
                start_line: 1,
                start_column: 13
            }
        },
        Token {
            data: TokenData::NumberLiteral(224.5f64),
            text: Cow::Borrowed("224.5"),
            location: TextLocation {
                start_char: 37,
                start_line: 1,
                start_column: 15
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 42,
                start_line: 1,
                start_column: 20
            }
        }
    });
}
