pub mod rpn;
pub mod token;

use crate::facts::Fact;
use std::io::Error;
use token::{Operand, Token};

#[derive(Copy, Clone, PartialEq)]
pub enum Side {
    Lhs,
    Rhs,
    Pending,
    Bidirectional,
}

// type Rule<'rule> = Vec<Token<'rule>>
#[derive(Default, Clone, Debug)]
pub struct Rule<'rule> {
    pub lhs: Vec<Token<'rule>>,
    pub rhs: Vec<Token<'rule>>,
}

impl<'rule> Rule<'rule> {
    pub fn new() -> Self {
        Rule {
            lhs: Vec::new(),
            rhs: Vec::new(),
        }
    }

    pub fn as_rpn(&mut self) -> Result<(), Error> {
        self.lhs = rpn::apply_on_vec(&self.lhs)?;
        self.lhs.reverse();
        self.rhs = rpn::apply_on_vec(&self.rhs)?;
        self.rhs.reverse();
        Ok(())
    }

    pub fn push(&mut self, side: Side, operand: Option<Operand>, fact: Option<&'rule Fact>) {
        if side == Side::Lhs {
            self.lhs.push(Token::new(operand, fact));
        } else {
            self.rhs.push(Token::new(operand, fact));
        }
    }

    pub fn implies_fact(&self, implied_fact: &Fact) -> bool {
        let mut is_prev_not = false;
        for token in self.rhs.iter() {
            if let Some(fact) = token.fact {
                if fact.letter == implied_fact.letter {
                    if is_prev_not {
                        fact.reverse_state.set(true);
                    }
                    return true;
                }
            }
            is_prev_not = false;
            if let Some(op) = token.operand {
                is_prev_not = op == Operand::Not;
            }
        }
        false
    }

    pub fn print(&self) {
        for token in &self.lhs {
            token.print();
        }
        print!("=> ");
        for token in &self.rhs {
            token.print();
        }
        println!();
    }
}
