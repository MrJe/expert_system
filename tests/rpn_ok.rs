pub mod common;
use common::run_test;

use std::io::Error;

/* *** OK TRUE *** */
#[test]
fn test_rpn_ok() -> Result<(), Error> {
    let expr: &str = "A + B";
    let rslt: &str = "A B +";
    run_test(expr, rslt, true)
}
#[test]
fn test_rpn_complexe() -> Result<(), Error> {
    let expr: &str = "(C ^ B + A) | ((!E | !B) ^ E)";
    let rslt: &str = "C B A + ^ E! B! | E ^ |";
    run_test(expr, rslt, true)
}
#[test]
fn test_rpn_00() -> Result<(), Error> {
    let expr: &str = "A ^ B + C | D";
    let rslt: &str = "A B C + D | ^";
    run_test(expr, rslt, true)
}
#[test]
fn test_rpn_01() -> Result<(), Error> {
    let expr: &str = "(A ^ B) + (C | D)";
    let rslt: &str = "A B ^ C D | +";
    run_test(expr, rslt, true)
}
#[test]
fn test_rpn_02() -> Result<(), Error> {
    let expr: &str = "A ^ (B + (C | D))";
    let rslt: &str = "A B C D | + ^";
    run_test(expr, rslt, true)
}
#[test]
fn test_rpn_03() -> Result<(), Error> {
    let expr: &str = "A + B | C ^ (A ^ !C)";
    let rslt: &str = "A B + C | A C ! ^ ^";
    run_test(expr, rslt, true)
}
#[test]
fn test_rpn_04() -> Result<(), Error> {
    let expr: &str = "((!(!(!(!(A + !B) ^ C) + D) | !E)))";
    let rslt: &str = "A B ! + ! C ^ ! D + ! E ! | !";
    run_test(expr, rslt, true)
}
#[test]
fn test_rpn_05() -> Result<(), Error> {
    let expr: &str = "A ^ B ^ C + E + D";
    let rslt: &str = "A B ^ C E + D + ^";
    run_test(expr, rslt, true)
}
#[test]
fn test_rpn_06() -> Result<(), Error> {
    let expr: &str = "A | B | C";
    let rslt: &str = "A B | C |";
    run_test(expr, rslt, true)
}
#[test]
fn test_rpn_07() -> Result<(), Error> {
    let expr: &str = "(A | B) ^ (C | D)";
    let rslt: &str = "A B | C D | ^";
    run_test(expr, rslt, true)
}
#[test]
fn test_rpn_08() -> Result<(), Error> {
    let expr: &str = "A + !B";
    let rslt: &str = "A B ! +";
    run_test(expr, rslt, true)
}
#[test]
fn test_rpn_09() -> Result<(), Error> {
    let expr: &str = "!!!A";
    let rslt: &str = "A ! ! !";
    run_test(expr, rslt, true)
}
#[test]
fn test_rpn_10() -> Result<(), Error> {
    let expr: &str = "!!(!A + !B)";
    let rslt: &str = "A ! B ! + ! !";
    run_test(expr, rslt, true)
}
#[test]
fn test_rpn_11() -> Result<(), Error> {
    let expr: &str = "A ^ !!(B | !C) + D";
    let rslt: &str = "A B C ! | ! ! D + ^";
    run_test(expr, rslt, true)
}
#[test]
fn test_rpn_12() -> Result<(), Error> {
    let expr: &str = "A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z";
    let rslt: &str = "A B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z |";
    run_test(expr, rslt, true)
}
