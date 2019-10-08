use crate::checker;
use crate::facts::Fact;
use crate::graph::{Graph, NodeIndex};
use crate::rules::{rule::{Side, token::{Token, Operand}}, Rules};

use std::io::{Error, ErrorKind};

fn push_operand<'a>(mut graph: Graph<Token<'a>>, token: Token<'a>, cur: &mut NodeIndex, side: Side) -> Result<Graph<Token<'a>>, Error> {
    match side {
        Side::Lhs => *cur = graph.insert_lhs(*cur, token)?,
        Side::Rhs => *cur = graph.insert_rhs(*cur, token)?,
        _ => panic!("Only lhs/rhs can be pushed in a graph. Code error"),
    }
    Ok(graph)
}

fn push_fact<'a>(mut graph: Graph<Token<'a>>, rules: &'a Rules, token: Token<'a>, fact: &Fact, cur: &mut NodeIndex, side: Side) -> Result<Graph<Token<'a>>, Error> {
    let sub_head = match side {
        Side::Lhs => graph.insert_lhs(*cur, token)?,
        Side::Rhs => graph.insert_rhs(*cur, token)?,
        _ => panic!("Only lhs/rhs can be pushed in a graph. Code error"),
    };
    if !fact.determined.get() {
        checker::infinite_rule_loop(&graph, sub_head, fact)?;
        graph = generate(graph, fact, sub_head, rules)?;
    }
    Ok(graph)
}

fn push_rec<'a>(mut graph: Graph<Token<'a>>, rules: &'a Rules, token: Token<'a>, cur: &mut NodeIndex) -> Result<Graph<Token<'a>>, Error> {
    match graph.get(*cur) {
        Some(mut node) => {
            if node.lhs.is_none() {
                if token.is_operand() {
                    graph = push_operand(graph, token, cur, Side::Lhs)?;
                } else if let Some(fact) = token.fact {
                    graph = push_fact(graph, rules, token, fact, cur, Side::Lhs)?;
                }
            } else if token.operand.is_some() && node.rhs.is_none() {
                graph = push_operand(graph, token, cur, Side::Rhs)?;
            } else {
                while node.parent.is_some() {
                    if node.rhs.is_some() || node.content.operand == Some(Operand::Not) {
                        *cur = node.parent.unwrap();
                    } else if token.is_operand() {
                        graph = push_operand(graph, token, cur, Side::Rhs)?;
                        break;
                    } else if let Some(fact) = token.fact {
                        graph = push_fact(graph, rules, token, fact, cur, Side::Rhs)?;
                        break;
                    }
                    node = graph.get(*cur).unwrap(); // danger
                }
            }
            Ok(graph)
        },
        None => {
            return Err(Error::new(
                ErrorKind::NotFound,
                "Tree builder: no current node to push operand",
            ))
        }
    }
}

pub fn generate<'a>(mut graph: Graph<Token<'a>>, queried: &Fact, mut cur: NodeIndex, rules: &'a Rules,) -> Result<Graph<Token<'a>>, Error> {
    for rule in rules.iter() {
        if rule.implies_fact(queried) {
            for token in rule.lhs.iter() {
                let token = *token;
                graph = push_rec(graph, rules, token, &mut cur)?;
            }
        }
    }
    Ok(graph)
}
