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

impl<'a> Token<'a> {
    pub fn new(operand: Option<Operand>, fact: Option<&Fact>) -> Token {
        Token { operand, fact }
    }

    pub fn new_op(operand: Operand) -> Token<'a> {
        Token {
            operand: Some(operand),
            fact: None,
        }
    }

    pub fn new_fact(fact: &'a Fact) -> Token<'a> {
        Token {
            operand: None,
            fact: Some(fact),
        }
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
            Some(fact) => fact.state.set(new_state),
            None => println!("set state on None"),
        }
    }

    pub fn get_state(&self) -> bool {
        match self.fact {
            Some(fact) => fact.state.get(),
            None => false,
        }
    }

    pub fn get_op_char(&self) -> char {
        if let Some(op) = self.operand {
            return match op {
                Operand::Not => '!',
                Operand::And => '+',
                Operand::Or => '|',
                Operand::Xor => '^',
                Operand::Opening => '(',
                Operand::Closing => ')',
            };
        }
        self.fact.unwrap().letter
    }

    pub fn get_token_char(&self) -> char {
        if let Some(fact) = self.fact {
            fact.letter
        } else {
            self.get_op_char()
        }
    }

    pub fn print(&self) {
        if let Some(fact) = self.fact {
            print!("{} ", fact.letter);
        } else if self.operand.is_some() {
            print!("{} ", self.get_op_char());
        } else {
            panic!("Empty Token");
        }
    }

    pub fn print_state(&self) {
        if let Some(fact) = self.fact {
            print!("{}({}) ", fact.state.get(), fact.letter);
        } else if self.operand.is_some() {
            print!("{} ", self.get_op_char());
        } else {
            panic!("Empty Token");
        }
    }

    pub fn cmp_tok(&self, tok: &Token) -> bool {
        if self.fact.is_some() && tok.fact.is_some() {
            return self.fact.unwrap().letter == tok.fact.unwrap().letter;
        } else if self.operand.is_some() && tok.operand.is_some() {
            return self.operand.unwrap() == tok.operand.unwrap();
        }
        false
    }
}
