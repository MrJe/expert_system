use super::token::{Operand, Token};
use super::Rule;
use std::io::{Error, ErrorKind};

impl<'rules> Rule<'rules> {
    fn solve_side(&self, side: &Vec<Token>) -> Result<bool, Error> {
        let mut facts: Vec<bool> = Vec::new();
        for token in side.iter() {
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

    fn solve_rhs(&self, lhs_value: bool) -> Result<bool, Error> {
        if self.rhs.len() == 1 {
            if let Some(fact) = self.rhs[0].fact {
                fact.state.set(lhs_value);
                fact.determined.set(true);
                return Ok(lhs_value)
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "no rhs fact"))
            }
        }
        let rhs_state: bool = self.solve_side(&self.rhs)?;
        Ok(rhs_state)
    }


    pub fn solve(&self) -> Result<(), Error> {
        let lhs_value: bool = self.solve_side(&self.lhs)?;
        println!("lhs = {}", lhs_value);
        let rhs_value: bool = self.solve_rhs(lhs_value)?;
        println!("rhs = {}", rhs_value);
        if self.is_equivalent == true && lhs_value != rhs_value {
            return Err(Error::new(ErrorKind::InvalidData, "Contradiction in rules"))
        }
        Ok(())
    }
}
