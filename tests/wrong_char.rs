pub mod common;
use common::run_test;

use std::io::Error;

/* *** Wrong char Error *** */
#[test]
fn test_rpn_char_01() -> Result<(), Error> {
    let expr: &str = "a ^ b + c | d";
    let rslt: &str = "a b c + ^ d |";
    run_test(expr, rslt, false)
}
#[test]
fn test_rpn_char_02() -> Result<(), Error> {
    let expr: &str = "1 ^ 2 + 3 | 4";
    let rslt: &str = "1 2 3 + ^ 4 |";
    run_test(expr, rslt, false)
}
#[test]
fn test_rpn_char_03() -> Result<(), Error> {
    let expr: &str = "A * B - C / D";
    let rslt: &str = "A B C - * D /";
    run_test(expr, rslt, false)
}
