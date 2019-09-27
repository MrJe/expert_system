use super::token::{Operand, Token};
use super::Rule;
use crate::facts::Fact;
use std::io::{Error, ErrorKind};

impl<'rules> Rule<'rules> {
    fn solve_lhs(&self) -> Result<bool, Error> {
        let mut facts: Vec<bool> = Vec::new();
        for token in self.lhs.iter() {
            if let Some(fact) = token.fact {
                facts.push(fact.state.get());
                print!("{}({}) ", fact.letter, fact.state.get());
            } else if let Some(op) = token.operand {
                print!(" {} ", token.get_op_char());
                match op {
                    Operand::Not => {
                        if let Some(state) = facts.last_mut() {
                            *state = !*state;
                        }
                    },
                    Operand::And => {
                        if let Some(last) = facts.pop() {
                            if let Some(state) = facts.last_mut() {
                                *state &= last;
                            }
                        }
                    },
                    Operand::Xor => {
                        if let Some(last) = facts.pop() {
                            if let Some(state) = facts.last_mut() {
                                *state ^= last;
                            }
                        }
                    },
                    Operand::Or => {
                        if let Some(last) = facts.pop() {
                            if let Some(state) = facts.last_mut() {
                                *state |= last;
                            }
                        }
                    },
                    _ => panic!("Solver: brackets found, should never happen"),
                }
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "Solver: empty token"))
            }
        }
        let result = facts.get(0);
        if facts.len() == 1 && result.is_some() {
            return Ok(*result.unwrap())
        } else {
            return Err(Error::new(ErrorKind::InvalidData, "Solver: something went wrong"))
        }
    }

    fn solve_rhs_aux(&self) -> Result<bool, Error> { // TODO this is the last function to do
        let mut facts: Vec<&Fact> = Vec::new();
        for token in self.rhs.iter() {
            if let Some(fact) = token.fact {
                facts.push(&fact);
            } else if let Some(op) = token.operand {
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "Solver: empty token"))
            }
        }
        if facts.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "Solver: rhs inequality"))
        }
        for fact in facts {
            fact.state.set(true);
        }
        Ok(true)
    }


//TODO : check rhs undetermined X/OR situation
    fn solve_rhs(&self) -> Result<bool, Error> { // lhs is  always true
        let rhs_state: bool = self.solve_rhs_aux()?;
        if self.is_equivalent == true && !rhs_state {
            return Err(Error::new(ErrorKind::InvalidData, "Solver: equivalence contradiction"))
        }
        Ok(rhs_state)
    }


    pub fn solve(&self) -> Result<(), Error> {
        let lhs_value: bool = self.solve_lhs()?;
        println!("lhs = {}", lhs_value);
        if lhs_value == true {
            let rhs_value: bool = self.solve_rhs()?;
            println!("rhs = {}", rhs_value);
        }
        Ok(())
    }
}
