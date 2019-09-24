pub mod token;

use crate::parser::Fact;
use token::{Operand, Token};

#[derive(Copy, Clone, PartialEq)]
pub enum Side {
    Lhs,
    Rhs,
    Pending,
    Bidirectional,
}

// type Rule<'a> = Vec<Token<'a>>
pub struct Rule<'a> {
    pub lhs: Vec<Token<'a>>,
    pub rhs: Vec<Token<'a>>,
    pub is_equivalent: bool,
}

impl<'a> Rule<'a> {
    pub fn new() -> Rule<'a> {
        Rule {
            lhs: Vec::new(),
            rhs: Vec::new(),
            is_equivalent: false,
        }
    }

    pub fn push(&mut self, side: &Side, operand: Option<Operand>, fact: Option<&'a Fact>) {
        if *side == Side::Lhs {
            self.lhs.push(Token::new(operand, fact));
        } else {
            self.rhs.push(Token::new(operand, fact));
        }
    }

    pub fn print(&self) {
        println!("Rule: (LHS)");
        for token in &self.lhs {
            println!(
                "\tToken: operand {:?}, fact: {:?}",
                token.operand, token.fact
            );
        }
        println!("Implies (=> RHS) ; <=> : {}", self.is_equivalent);
        for token in &self.rhs {
            println!(
                "\tToken: operand {:?}, fact: {:?}",
                token.operand, token.fact
            );
        }
    }
}
