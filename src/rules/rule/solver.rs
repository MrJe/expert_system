use super::token::{Operand, Token};
use super::Rule;
use std::io::Error; //, ErrorKind};

impl<'rules> Rule<'rules> {
    pub fn solve(&self) -> Result<(), Error> {
        let mut rules_cpy: Vec<Token> = self.lhs.clone();
        let mut i = 0;
        while i < rules_cpy.len() {
            let token = rules_cpy[i];
            if let Some(op) = token.operand {
                match op {
                    Operand::Not => {
                        &rules_cpy.remove(i);
                        let prev = rules_cpy[i - 1].fact;
                        if let Some(prev) = prev {
                            prev.tmp_state.set(prev.state.get());
                        }
                    }
                    Operand::And => {
                        rules_cpy.remove(i);
                        let prev = rules_cpy.remove(i - 1);
                        if let Some(prev) = prev.fact {
                            let edited = rules_cpy[i - 2].fact;
                            if let Some(edited) = edited {
                                let new_state = prev.state.get() & edited.state.get();
                                edited.tmp_state.set(new_state);
                            }
                        }
                    }
                    Operand::Xor => {
                        rules_cpy.remove(i);
                        let prev = rules_cpy.remove(i - 1);
                        if let Some(prev) = prev.fact {
                            let edited = rules_cpy[i - 2].fact;
                            if let Some(edited) = edited {
                                let new_state = prev.state.get() ^ edited.state.get();
                                edited.tmp_state.set(new_state);
                            }
                        }
                    }
                    Operand::Or => {
                        rules_cpy.remove(i);
                        let prev = rules_cpy.remove(i - 1);
                        if let Some(prev) = prev.fact {
                            let edited = rules_cpy[i - 2].fact;
                            if let Some(edited) = edited {
                                let new_state = prev.state.get() | edited.state.get();
                                edited.tmp_state.set(new_state);
                            }
                        }
                    }
                    _ => panic!("Solver: brackets found, should never happen"),
                }
            }
            i += 1;
        }
        Ok(())
    }
}
