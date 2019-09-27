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

fn tokenise_str<'rule>(arg: &str, facts: &'rule Vec<Fact>) -> Vec<Token<'rule>> {
    let mut ret: Vec<Token> = Vec::new();

    for c in arg.chars() {
        match c {
            ' ' | '\t'  => continue,
            'A'..='Z'   => ret.push(Token::new(None, Some(&facts[c as usize - 65]))),
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
    let mut i = exp.len();
    if i != res.len() {
        return false
    }
    while i > 0 {
        i -= 1;
        if exp[i].cmp_tok(&res[i]) == false {
            return false
        }
    }
    true
}

fn run_test(expr: &str, rslt: &str, ass: bool) -> Result<(), Error> {
    let facts = vec![
        Fact::new('A'),
        Fact::new('B'),
        Fact::new('C'),
        Fact::new('D'),
        Fact::new('E'),
        Fact::new('F'),
        Fact::new('G'),
        Fact::new('H'),
        Fact::new('I'),
        Fact::new('J'),
        Fact::new('K'),
        Fact::new('L'),
        Fact::new('M'),
        Fact::new('N'),
        Fact::new('O'),
        Fact::new('P'),
        Fact::new('Q'),
        Fact::new('R'),
        Fact::new('S'),
        Fact::new('T'),
        Fact::new('U'),
        Fact::new('V'),
        Fact::new('W'),
        Fact::new('X'),
        Fact::new('Y'),
        Fact::new('Z'),
    ];
    let exptok: Vec<Token> = tokenise_str(&expr, &facts);
    let exptok: Vec<Token> = apply_on_vec(&exptok)?;
    let rsltok: Vec<Token> = tokenise_str(&rslt, &facts);

    print!("exptok: ");
    print_tokens(&exptok);
    print!("rsltok: ");
    print_tokens(&rsltok);

    assert_eq!(cmp_tokens(exptok, rsltok), ass);
    Ok(())
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
#[should_panic]
fn test_rpn_ko_op_miss() {
    let expr: &str = "A + B";
    let rslt: &str = "A + B";
    let _ = run_test(expr, rslt, true);
}
#[test]
#[should_panic]
fn test_rpn_ko_bad_op() {
    let expr: &str = "A + B";
    let rslt: &str = "A B ^";
    let _ = run_test(expr, rslt, true);
}
#[test]
#[should_panic]
fn test_rpn_ko_letter_a() {
    let expr: &str = "A + B";
    let rslt: &str = "A A +";
    let _ = run_test(expr, rslt, true);
}
#[test]
#[should_panic]
fn test_rpn_ko_letter_b() {
    let expr: &str = "A + B";
    let rslt: &str = "B B +";
    let _ = run_test(expr, rslt, true);
}
#[test]
fn test_rpn_brackets_01() -> Result<(), Error> {
    let expr: &str = "A ^ )B + C | D";
    let rslt: &str = "A B C + ^ D |";
    let _ = run_test(expr, rslt, false);
    Ok(())
}
#[test]
fn test_rpn_brackets_02() -> Result<(), Error> {
    let expr: &str = "(A ^ B + C | D";
    let rslt: &str = "A B C + ^ D |";
    let _ = run_test(expr, rslt, false);
    Ok(())
}
#[test]
fn test_rpn_brackets_03() -> Result<(), Error> {
    let expr: &str = "A ^ B + C | D)";
    let rslt: &str = "A B C + ^ D |";
    let _ = run_test(expr, rslt, false);
    Ok(())
}
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
    let rslt: &str = "A B C + ^ D |";
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
