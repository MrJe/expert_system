pub mod rule;

use crate::checker;
use crate::facts::Facts;
use crate::options::Options;
use core::slice::Iter;
use rule::{token::Operand, Rule, Side};
use std::io::{Error, ErrorKind};

#[derive(Default)]
pub struct Rules<'rules>(Vec<Rule<'rules>>);

impl<'rules> Rules<'rules> {
    pub fn new() -> Self {
        Rules(Vec::new())
    }

    pub fn set_rule(
        &mut self,
        facts: &'rules Facts,
        line: &str,
        options: &Options,
    ) -> Result<(), Error> {
        let mut side = Side::Lhs;
        let mut rule = Rule::new();
        let mut is_equivalent = false;

        for c in line.chars() {
            if side == Side::Pending || side == Side::Bidirectional {
                if side == Side::Bidirectional {
                    is_equivalent = true;
                }
                checker::impliance(&mut side, c)?;
                continue;
            }
            if c.is_whitespace() {
                continue;
            } else if c.is_uppercase() {
                rule.push(side, None, Some(facts.get(c)));
            } else {
                match c {
                    '(' => rule.push(side, Some(Operand::Opening), None),
                    ')' => rule.push(side, Some(Operand::Closing), None),
                    '!' => rule.push(side, Some(Operand::Not), None),
                    '|' => rule.push(side, Some(Operand::Or), None),
                    '^' => rule.push(side, Some(Operand::Xor), None),
                    '+' => rule.push(side, Some(Operand::And), None),
                    '#' => {
                        if options.comment && !options.file {
                            println!("{}", line);
                        }
                        break;
                    }
                    '<' | '=' => checker::impliance(&mut side, c)?,
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            format!("Rules: unexpected char (at {})", line),
                        ))
                    }
                };
            }
        }
        if side != Side::Rhs {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Rules: no impliance (at {})", line),
            ));
        }
        checker::rule_composition(&rule.lhs, line)?;
        checker::rule_composition(&rule.rhs, line)?;
        if is_equivalent {
            let mut implicit_rule = Rule::new();
            implicit_rule.lhs = rule.rhs.clone();
            implicit_rule.rhs = rule.lhs.clone();
            self.0.push(implicit_rule);
        }
        self.0.push(rule);
        Ok(())
    }

    pub fn iter(&self) -> Iter<Rule<'rules>> {
        self.0.iter()
    }

    pub fn as_reverse_polish_notation(&mut self) -> Result<(), Error> {
        for rule in self.0.iter_mut() {
            rule.as_rpn()?;
        }
        Ok(())
    }

    pub fn print(&self) {
        println!("PRINTING RULES");
        for rule in &self.0 {
            rule.print();
        }
        println!();
    }
}
