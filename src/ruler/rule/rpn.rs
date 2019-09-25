use super::token::{Operand, Token};

pub fn apply_on_vec<'rule>(tokens: &Vec<Token<'rule>>) -> Vec<Token<'rule>> {
    let mut ret : Vec<Token>    = Vec::new();
    let mut tmp : Vec<Operand>  = Vec::new();

    for token in tokens {
        if token.fact.is_some() {
            ret.push(token.clone());
            continue;
        }
        match token.operand {
            None        => panic!("Token without fact or operand."),
            Some(op)    => sort_operand(&mut ret, &mut tmp, op),
        }
    }
    while tmp.is_empty() == false {
        let last = *tmp.last().unwrap();
        if last == Operand::Opening {
            println!("Error with (), too much '('");
        }
        ret.push(Token::new(Some(last), None));
        tmp.pop();
    }
    ret
}

fn op_lvl(op: Operand) -> u8 {
    match op {
        Operand::Not => 1,
        Operand::And => 2,
        Operand::Xor => 3,
    	Operand::Or  => 4,
        _            => 5,
    }
}

fn unstack_to_opening(ret: &mut Vec<Token>, tmp: &mut Vec<Operand>) {
    while tmp.is_empty() == false {
        let last = *tmp.last().unwrap();
        tmp.pop();

        if last == Operand::Opening {
            break;
        }
        else if tmp.is_empty() {
            println!("Error with (), '(' is missing.")
        }
        ret.push(Token::new(Some(last), None));
    }
}

fn unstack_with_lvl(ret: &mut Vec<Token>, tmp: &mut Vec<Operand>, op: Operand) {
    while tmp.is_empty() == false {
        let last = *tmp.last().unwrap();
        tmp.pop();

        ret.push(Token::new(Some(last), None));
        if tmp.is_empty() == false
            && op_lvl(op) <= op_lvl(*tmp.last().unwrap()) {
            break;
        }
    }
}

fn sort_operand(ret: &mut Vec<Token>, tmp: &mut Vec<Operand>, op: Operand) {
    if op == Operand::Closing {
        if tmp.is_empty() {
            println!("Error with (), ')' without '('")
        }
        unstack_to_opening(ret, tmp);
    }
    else if op != Operand::Opening && tmp.is_empty() == false &&
            op_lvl(op) > op_lvl(*tmp.last().unwrap()) {
        unstack_with_lvl(ret, tmp, op);
        tmp.push(op.clone());
    }
    else {
        tmp.push(op.clone());
    }
}
