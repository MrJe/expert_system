use super::token::{Operand, Token};
use std::io::{Error, ErrorKind};

pub fn apply_on_vec<'rule>(tokens: &[Token<'rule>]) -> Result<Vec<Token<'rule>>, Error> {
    let mut ret: Vec<Token> = Vec::new();
    let mut tmp: Vec<Operand> = Vec::new();

    for token in tokens {
        if token.fact.is_some() {
            ret.push(token.clone());
            continue;
        }
        match token.operand {
            None => panic!("Token without fact or operand."),
            Some(op) => sort_operand(&mut ret, &mut tmp, op)?,
        }
    }
    while !tmp.is_empty() {
        if let Some(last) = tmp.last() {
            if *last == Operand::Opening {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Rpn: brackets do not match (closing missing)",
                ));
            }
            ret.push(Token::new_op(*last));
            tmp.pop();
        } else {
            panic!("apply_on_vec(): tmp contain invalid Operand");
        }
    }
    Ok(ret)
}

fn op_priority(op: Operand) -> u8 {
    match op {
        Operand::Not => 1,
        Operand::And => 2,
        Operand::Or  => 3,
        Operand::Xor => 4,
        _ => 5,
    }
}

fn unstack_to_opening(ret: &mut Vec<Token>, tmp: &mut Vec<Operand>) -> Result<(), Error> {
    while !tmp.is_empty() {
        let last = *tmp.last().unwrap();
        tmp.pop();

        if last == Operand::Opening {
            break;
        } else if tmp.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Rpn: brackets do not match (opening missing)",
            ));
        }
        ret.push(Token::new_op(last));
    }
    Ok(())
}

fn unstack_with_lvl(ret: &mut Vec<Token>, tmp: &mut Vec<Operand>, op: Operand) {
    while !tmp.is_empty() {
        let last = *tmp.last().unwrap();
        tmp.pop();

        ret.push(Token::new_op(last));
        if let Some(last) = tmp.last() {
            if op_priority(op) < op_priority(*last) {
                break;
            }
        }
    }
}

fn sort_operand(ret: &mut Vec<Token>, tmp: &mut Vec<Operand>, op: Operand) -> Result<(), Error> {
    if op == Operand::Closing {
        if tmp.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Rpn: brackets do not match (opening missing)",
            ));
        }
        unstack_to_opening(ret, tmp)?;
        return Ok(());
    }
    if op != Operand::Opening && op != Operand::Not
        && !tmp.is_empty()
        && op_priority(op) >= op_priority(*tmp.last().unwrap())
    {
        unstack_with_lvl(ret, tmp, op);
    }
    tmp.push(op);
    Ok(())
}
