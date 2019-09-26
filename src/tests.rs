use crate::rules::rule::token::{Operand, Token};
use crate::rules::rule::rpn::apply_on_vec;
use crate::facts::Fact;
use std::io::Error;

fn get_operand(c: char) -> Operand {
    match c {
        '!' => Operand::Not,
        '+' => Operand::And,
        '^' => Operand::Xor,
        '|' => Operand::Or,
        '(' => Operand::Opening,
        _   => Operand::Closing,
    }
}

fn tokenise_str(arg: &str) -> Vec<Token> {
    let mut ret: Vec<Token> = Vec::new();

    for c in arg.chars() {
        match c {
            ' ' | '\t'  => continue,
            'A'..='Z'   => ret.push(Token::new(None, Some(&Fact::new(c)))),
            _           => ret.push(Token::new(Some(get_operand(c)), None)),
        }
    }
    ret
}

fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        if token.fact.is_some() {
            print!("{} ", token.fact.as_ref().unwrap().letter);
        }
        else {
            print!("{} ", token.get_op_char());
        }
    }
    println!("");
}

fn cmp_tokens(exp: Vec<Token>, res: Vec<Token>) -> bool {
    if exp.len() != res.len() {
        return false
    }
    true
}

#[test]
#[should_panic]
fn test_ko() {
    assert_eq!(true, false);
}

#[test]
fn test_ok() {
    assert_eq!(true, true);
}

#[test]
fn test_rpn() -> Result<(), Error> {
    let expr: &str = "(C ^ B + A) | ((!E | !B) ^ E)";
    let rslt: &str = "C B A + ^ E! B! | E ^ |";

    let exptok: Vec<Token> = tokenise_str(&expr);
    let exptok: Vec<Token> = apply_on_vec(&exptok)?;
    let rsltok: Vec<Token> = tokenise_str(&rslt);

    print_tokens(&exptok);
    print_tokens(&rsltok);

    assert_eq!(cmp_tokens(exptok, rsltok), true);
    Ok(())
}
