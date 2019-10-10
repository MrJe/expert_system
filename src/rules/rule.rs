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
        let mut is_not = false;
        for token in self.rhs.iter() {
            if let Some(fact) = token.fact {
                if fact.letter == implied_fact.letter {
                    fact.reverse_state.set(is_not);
                    return true;
                }
                is_not = false;
            }
            if let Some(op) = token.operand {
                if op == Operand::Not {
                    is_not = !is_not;
                } else {
                    is_not = false;
                }
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
