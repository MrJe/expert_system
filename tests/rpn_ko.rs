pub mod common;
use common::run_test;

use std::io::Error;

/* *** KO FALSE *** */
#[test]
fn test_rpn_ko_op_miss() -> Result<(), Error> {
    let expr: &str = "A + B";
    let rslt: &str = "A + B";
    run_test(expr, rslt, false)
}
#[test]
fn test_rpn_ko_bad_op() -> Result<(), Error> {
    let expr: &str = "A + B";
    let rslt: &str = "A B ^";
    run_test(expr, rslt, false)
}
#[test]
fn test_rpn_ko_letter_a() -> Result<(), Error> {
    let expr: &str = "A + B";
    let rslt: &str = "A A +";
    run_test(expr, rslt, false)
}
#[test]
fn test_rpn_ko_letter_b() -> Result<(), Error> {
    let expr: &str = "A + B";
    let rslt: &str = "B B +";
    run_test(expr, rslt, false)
}
