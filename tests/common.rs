use lib::rules::rule::token::{Operand, Token};
use lib::rules::rule::rpn::apply_on_vec;
use lib::facts::Fact;
// use lib::expert_system::run_ep;

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

pub fn run_test(expr: &str, rslt: &str, ass: bool) -> Result<(), Error> {
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
    let rsltok: Vec<Token> = tokenise_str(&rslt, &facts);

    let exptok_rpn = apply_on_vec(&exptok);
    let mut exptok: Vec<Token> = Vec::new();

    match exptok_rpn {
        Ok(vec)  => exptok = vec,
        Err(er)  => {
            if ass == true {
                return Err(er)
            }
        }
    }
    assert_eq!(cmp_tokens(exptok, rsltok), ass);
    Ok(())
}
