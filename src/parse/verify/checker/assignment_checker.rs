//! Checks for redundant assignments (`x = x`).

use lex::Token;
use parse::expression::{Identifier, Assignment, Expression};
use parse::verify::{ExpressionChecker, VerifyError, ErrorCollector};

/// Reports warnings for redundant assignments (`x = x`)
pub struct AssignmentChecker { }
impl ExpressionChecker for AssignmentChecker {
    fn check_assignment(&mut self, errors: &mut ErrorCollector, assign: &Assignment) {
        let var_name = assign.lvalue.get_name();

        match *assign.rvalue {
            Expression::VariableRef(ref r_var) => {
                if var_name == r_var.get_name() {
                    let err_text = format!("Redundant assignment of {} to itself", var_name);
                    errors.add_error(VerifyError::new(assign.lvalue.token.clone(), vec![], err_text));
                }
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use lex::{Token, TokenData, TextLocation};
    use parse::tests::make_parser;
    use parse::verify::{ExpressionChecker, ErrorCollector, VerifyError};
    use super::AssignmentChecker;
    use parse::verify::checker::tests::verify_checker_errors;

    #[test]
    fn it_finds_redundant_assign() {
        let input = "x = x";
        let checker = AssignmentChecker { };
        let expected = vec![
            VerifyError::new(Token {
                location: TextLocation { index: 0, line: 0, column: 0 },
                text: Cow::Borrowed("x"),
                data: TokenData::Ident
            },
            vec![], "Redundant assignment of x to itself".to_string())
        ];
        verify_checker_errors(input, checker, expected);
    }

    #[test]
    fn it_allows_complex_assign() {
        let input = "x = y";
        let checker = AssignmentChecker { };
        let expected = vec![];
        verify_checker_errors(input, checker, expected);
    }

    #[test]
    fn it_allows_different_var_assign() {
        let input = "x = x + 1";
        let checker = AssignmentChecker { };
        let expected = vec![];
        verify_checker_errors(input, checker, expected);
    }

    #[test]
    fn it_allows_assign_op() {
        let input = "x += x"; // x = x + x
        let checker = AssignmentChecker { };
        let expected = vec![];
        verify_checker_errors(input, checker, expected);
    }

}
