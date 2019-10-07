use crate::facts::{Fact, Facts};
use crate::graph::{Graph, NodeIndex};
use crate::rules::rule::{token::Token, Side};

use std::io::{Error, ErrorKind};

pub fn solved_queries(facts: &Facts) -> Result<(), Error> {
    if !facts.is_stable {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "(Solved) Queries: unstable ruleset",
        ));
    }
    for fact in facts.fact_arr.iter() {
        if fact.queried.get() && !fact.determined.get() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "(Solved) Queries: undetermined fact",
            ));
        }
    }
    Ok(())
}

pub fn impliance(side: &mut Side, c: char) -> Result<(), Error> {
    if *side == Side::Rhs {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Rules: <, =, or > oversupplied",
        ));
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

pub fn rule_composition(tokens: &[Token], line: &str) -> Result<(), Error> {
    let mut last = &Token::new(None, None);
    for token in tokens {
        if !last.is_empty() {
            if token.is_fact() && last.is_fact() {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Rules: contiguous facts (at {})", line),
                ));
            }
            if token.is_operand() && !token.is_cumulable() && last.is_operand() {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Rules: contiguous operands (at {})", line),
                ));
            }
        }
        if !token.is_cumulable() {
            last = token;
        }
    }
    Ok(())
}

pub fn infinite_rule_loop<'a>(
    graph: &Graph<Token<'a>>,
    mut cur: NodeIndex,
    ref_fact: &Fact,
) -> Result<(), Error> {
    match graph.get(cur) {
        Some(mut node) => {
            while node.parent.is_some() {
                cur = node.parent.unwrap();
                node = graph.get(cur).unwrap();
                if let Some(fact) = node.content.fact {
                    if fact.letter == ref_fact.letter {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            "Tree builder (inf checker): INFINITE LOOP",
                        ));
                    }
                }
            }
            Ok(())
        }
        None => Err(Error::new(
            ErrorKind::NotFound,
            "Tree builder (inf checker): no current node",
        )),
    }
}
