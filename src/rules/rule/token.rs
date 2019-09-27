use crate::facts::Fact;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Operand {
    Not,
    And,
    Xor,
    Or,
    Opening,
    Closing,
}

#[derive(Copy, Clone, Debug)]
pub struct Token<'a> {
    pub operand: Option<Operand>,
    pub fact: Option<&'a Fact>,
}

impl Token<'_> {
    pub fn new(operand: Option<Operand>, fact: Option<&Fact>) -> Token {
        Token { operand, fact }
    }

    pub fn is_empty(&self) -> bool {
        self.fact.is_none() && self.operand.is_none()
    }

    pub fn is_fact(&self) -> bool {
        self.fact.is_some()
    }

    pub fn is_operand(&self) -> bool {
        self.operand.is_some()
    }

    pub fn is_cumulable(&self) -> bool {
        if let Some(op) = self.operand {
            return op == Operand::Not || op == Operand::Opening || op == Operand::Closing;
        }
        false
    }

    pub fn set_state(&self, new_state: bool) {
        match self.fact {
            Some(fact) => {
                fact.state.set(new_state);
                if fact.state.get() != fact.tmp_state.get() {
                    panic!(
                        "fact.set_state() should not be called when state != tmp_state, bad design"
                    );
                }
                fact.tmp_state.set(new_state);
            }
            None => println!("set state on None"),
        }
    }

    pub fn set_tmp_state(&self, new_state: bool) {
        match self.fact {
            Some(fact) => fact.tmp_state.set(new_state),
            None => println!("set state on None"),
        }
    }

    pub fn get_op_char(&self) -> char {
        if let Some(op) = self.operand {
            return match op {
                Operand::Not => '!',
                Operand::And => '+',
                Operand::Xor => '^',
                Operand::Or => '|',
                Operand::Opening => '(',
                Operand::Closing => ')',
            };
        }
        '@'
    }

    pub fn cmp_tok(&self, tok: &Token) -> bool {
        if self.fact.is_some() && tok.fact.is_some() {
            if self.fact.unwrap().letter == tok.fact.unwrap().letter {
                    return true
            } else {
                return false
            }
        } else if self.operand.is_some() && tok.operand.is_some() {
            if self.operand.unwrap() == tok.operand.unwrap() {
                return true
            } else {
                return false
            }
        }
        false
    }
}
