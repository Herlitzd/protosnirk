//! Checkers used to verify the expression trees created by the parser.
//!
//! Checkers have two purposes: check the parse tree for semantic errors,
//! (attemtped mutation of immutable variable) and build up representations
//! (such as the symbol table) of the parsed program to be used by the compiler.
//!
//! The verifiers in this module will build structures from the `build` module.
mod symbol_checker;
mod usage_checker;
mod assignment_checker;

pub use self::symbol_checker::SymbolTableChecker;
pub use self::usage_checker::UsageChecker;
pub use self::assignment_checker::AssignmentChecker;
pub use self::constant_checker::ConstantAssembler;

#[cfg(test)]
mod tests {
    use parse::verify::{ExpressionChecker, ErrorCollector, VerifyError};
    use parse::tests::make_parser;

    pub fn verify_checker_errors<E: ExpressionChecker>(input: &'static str,
                                                       mut checker: E,
                                                       expected: Vec<VerifyError>) {
        let mut parser = make_parser(input);
        let block = parser.block().unwrap();
        let mut verifier = ErrorCollector::new();
        checker.check_block(&mut verifier, &block);
        assert_eq!(verifier.get_errors(), &*expected);
    }
}
mod constant_checker;
