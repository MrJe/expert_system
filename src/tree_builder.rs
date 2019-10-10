use crate::checker;
use crate::facts::Fact;
use crate::graph::{Graph, NodeIndex};
use crate::rules::{
    rule::{
        token::{Operand, Token},
        Side,
    },
    Rules,
};
use crate::solver;

use std::io::{Error, ErrorKind};

fn push_operand<'a>(
    mut graph: Graph<Token<'a>>,
    token: Token<'a>,
    cur: &mut NodeIndex,
    side: Side,
) -> Result<Graph<Token<'a>>, Error> {
    match side {
        Side::Lhs => *cur = graph.insert_lhs(*cur, token)?,
        Side::Rhs => *cur = graph.insert_rhs(*cur, token)?,
        _ => panic!("Only lhs/rhs can be pushed in a graph. Code error"),
    }
    Ok(graph)
}

fn push_fact<'a>(
    mut graph: Graph<Token<'a>>,
    rules: &'a Rules,
    token: Token<'a>,
    fact: &'a Fact,
    cur: &mut NodeIndex,
    side: Side,
) -> Result<Graph<Token<'a>>, Error> {
    let sub_head = match side {
        Side::Lhs => graph.insert_lhs(*cur, token)?,
        Side::Rhs => graph.insert_rhs(*cur, token)?,
        _ => panic!("Only lhs/rhs can be pushed in a graph. Code error"),
    };
    if !fact.determined.get() {
        match checker::infinite_rule_loop(&graph, sub_head, fact) {
            Ok(()) => graph = generate(graph, rules, fact, sub_head)?,
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    println!("WTF, {:?}", e);
                    return Err(e)
                }
                fact.state.set(false);
            }
        }
    }
    Ok(graph)
}

fn push_rec<'a>(
    mut graph: Graph<Token<'a>>,
    rules: &'a Rules,
    token: Token<'a>,
    cur: &mut NodeIndex,
) -> Result<Graph<Token<'a>>, Error> {
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
                    if let Some(tmp) = graph.get(*cur) {
                        node = tmp;
                    } else {
                        return Err(Error::new(
                            ErrorKind::NotFound,
                            "Tree builder: no parent node found",
                        ))
                    }
                }
            }
            Ok(graph)
        }
        None => Err(Error::new(
            ErrorKind::NotFound,
            "Tree builder: no current node to push operand",
        )),
    }
}

pub fn generate<'a>(
    mut graph: Graph<Token<'a>>,
    rules: &'a Rules,
    queried: &'a Fact,
    mut cur: NodeIndex,
) -> Result<Graph<Token<'a>>, Error> {
    let saved_len = cur;
    for rule in rules.iter() {
        if rule.implies_fact(queried) {
            if let Some(mut false_node) = graph.get_mut(saved_len) {
                false_node.lhs = None;
                graph.truncate(saved_len + 1);
                cur = saved_len;
            }
            for token in rule.lhs.iter() {
                let token = *token;
                graph = push_rec(graph, rules, token, &mut cur)?;
            }
            let solved_branch = solver::tree_solver(&graph, saved_len)?;
            if solved_branch {
                queried.set_solved(solved_branch);
                break;
            }
        }
    }
    if !queried.determined.get() {
        queried.determined.set(true);
    }
    Ok(graph)
}
