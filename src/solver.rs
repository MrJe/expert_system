mod rule;

use crate::parser::Facts;
use rule::{token::Operand, token::Token, Rule, Side};
use std::io::{Error, ErrorKind};

pub struct Solver<'solver> {
    pub rules: Vec<Rule<'solver>>,
}

impl<'solver> Solver<'solver> {
    pub fn new() -> Solver<'solver> {
        Solver { rules: Vec::new() }
    }

    fn impliance_checker(side: &mut Side, &c: &char) -> Result<(), Error> {
        if *side == Side::Rhs {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Rules: <, =, or > oversupplied",
            ))
        }
        let side_cpy: Side = *side;
        match c {
            '=' => *side = Side::Pending,
            '<' => *side = Side::Bidirectional,
            '>' => *side = Side::Rhs,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Rules: impliance wrong format",
                ))
            }
        };
        if side_cpy == *side {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Rules: impliance wrong format",
            ));
        }
        Ok(())
    }

    fn rule_composition_checker(tokens: &Vec<Token>, line: &str) -> Result<(), Error> {
        let mut last = &Token::new(None, None);
        for token in tokens {
            if !last.is_empty() {
                if token.is_fact() && last.is_fact() {
                    return Err(Error::new(ErrorKind::InvalidData, format!("Rules: contiguous facts (at {})", line)))
                }
                if token.is_operand() && !token.is_cumulable() && last.is_operand() {
                    return Err(Error::new(ErrorKind::InvalidData, format!("Rules: contiguous operands (at {})", line)))
                }
            }
            if !token.is_cumulable() {
                last = token;
            }
        }
        Ok(())
    }

    pub fn set_rule(&mut self, facts: &'solver Facts, line: &str) -> Result<(), Error> {
        let mut side = Side::Lhs;
        let mut rule = Rule::new();

        for c in line.chars() {
            if side == Side::Pending || side == Side::Bidirectional {
                if side == Side::Bidirectional {
                    rule.is_equivalent = true;
                }
                Solver::impliance_checker(&mut side, &c)?;
                continue;
            }
            if c.is_whitespace() {
                continue;
            } else if c.is_uppercase() {
                rule.push(&side, None, Some(facts.get(c)));
            } else {
                match c {
                    '(' => rule.push(&side, Some(Operand::Opening), None),
                    ')' => rule.push(&side, Some(Operand::Closing), None),
                    '!' => rule.push(&side, Some(Operand::Not), None),
                    '|' => rule.push(&side, Some(Operand::Or), None),
                    '^' => rule.push(&side, Some(Operand::Xor), None),
                    '+' => rule.push(&side, Some(Operand::And), None),
                    '#' => break,
                    '<' | '=' => Solver::impliance_checker(&mut side, &c)?,
                    _ => return Err(Error::new(ErrorKind::InvalidData, format!("Rules: unexpected char (at {})", line))),
                };
            }
        }
        if side != Side::Rhs {
            return Err(Error::new(ErrorKind::InvalidData, format!("Rules: no impliance (at {})", line)));
        }
        Solver::rule_composition_checker(&rule.lhs, line)?;
        Solver::rule_composition_checker(&rule.rhs, line)?;
        self.rules.push(rule);
        Ok(())
    }

    pub fn print(&self) {
        println!("PRINTING RULES");
        for rule in &self.rules {
            rule.print();
        }
    }
}
