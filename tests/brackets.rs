pub mod common;
use common::run_test;

use std::io::Error;

/* *** Brackets Error *** */
#[test]
fn test_rpn_brackets_01() -> Result<(), Error> {
    let expr: &str = "A ^ )B + C | D";
    let rslt: &str = "A B C + ^ D |";
    run_test(expr, rslt, false)
}
#[test]
fn test_rpn_brackets_02() -> Result<(), Error> {
    let expr: &str = "(A ^ B + C | D";
    let rslt: &str = "A B C + ^ D |";
    run_test(expr, rslt, false)
}
#[test]
fn test_rpn_brackets_03() -> Result<(), Error> {
    let expr: &str = "A ^ B + C | D)";
    let rslt: &str = "A B C + ^ D |";
    run_test(expr, rslt, false)
}
#[test]
fn test_rpn_brackets_04() -> Result<(), Error> {
    let expr: &str = "A (^ B + C |) D";
    let rslt: &str = "A B C + ^ D |";
    run_test(expr, rslt, false)
}
