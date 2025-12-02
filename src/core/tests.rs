//! Shell test conditional expressions

use super::{ExecutionParameters, Shell, error, extendedtests};

/// Evaluate the given test expression within the provided shell and
/// execution context. Returns true if the expression evaluates to true,
/// false otherwise.
///
/// # Arguments
///
/// * `expr` - The test expression to evaluate.
/// * `shell` - The shell context in which to evaluate the expression.
/// * `params` - The execution parameters to use during evaluation.
pub fn eval_expr(
    expr: &crate::parser::ast::TestExpr,
    shell: &mut Shell,
    params: &ExecutionParameters,
) -> Result<bool, error::Error> {
    match expr {
        crate::parser::ast::TestExpr::False => Ok(false),
        crate::parser::ast::TestExpr::Literal(s) => Ok(!s.is_empty()),
        crate::parser::ast::TestExpr::And(left, right) => {
            Ok(eval_expr(left, shell, params)? && eval_expr(right, shell, params)?)
        }
        crate::parser::ast::TestExpr::Or(left, right) => {
            Ok(eval_expr(left, shell, params)? || eval_expr(right, shell, params)?)
        }
        crate::parser::ast::TestExpr::Not(expr) => Ok(!eval_expr(expr, shell, params)?),
        crate::parser::ast::TestExpr::Parenthesized(expr) => eval_expr(expr, shell, params),
        crate::parser::ast::TestExpr::UnaryTest(op, operand) => {
            extendedtests::apply_unary_predicate_to_str(op, operand, shell, params)
        }
        crate::parser::ast::TestExpr::BinaryTest(op, left, right) => {
            extendedtests::apply_binary_predicate_to_strs(op, left.as_str(), right.as_str(), shell)
        }
    }
}
