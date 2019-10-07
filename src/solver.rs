use crate::checker;
use crate::facts::Fact;
use crate::graph::{Graph, NodeIndex};
use crate::print::print_tree_to_file;
use crate::rules::{rule::token::{Token, Operand}, Rules};

use std::io::{Error, ErrorKind};

fn push_operand_rec<'a>(mut graph: Graph<Token<'a>>, token: Token<'a>, cur: &mut NodeIndex) -> Result<Graph<Token<'a>>, Error> {
    match graph.get(*cur) {
        Some(mut node) => {
            if node.lhs.is_none() {
                *cur = graph.insert_lhs(*cur, token)?;
            } else if node.rhs.is_none() {
                *cur = graph.insert_rhs(*cur, token)?;
            } else {
                while node.parent.is_some() {
                    if node.rhs.is_some() || node.content.operand == Some(Operand::Not) {
                        *cur = node.parent.unwrap();
                    } else {
                        *cur = graph.insert_rhs(*cur, token)?;
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

fn push_fact_rec<'a>(mut graph: Graph<Token<'a>>, rules: &'a Rules, token: Token<'a>, fact: &Fact, cur: &mut NodeIndex) -> Result<Graph<Token<'a>>, Error> {
    match graph.get(*cur) {
        Some(mut node) => {
            if node.lhs.is_none() {
                if fact.determined.get() == true {
                    graph.insert_lhs(*cur, token)?;
                } else {
                    let sub_head = graph.insert_lhs(*cur, token)?;
                    checker::infinite_rule_loop(&graph, sub_head, fact)?;
                    graph = generate_tree(graph, fact, sub_head, rules)?;
                }
            } else {
                while node.parent.is_some() {
                    if node.rhs.is_some() || node.content.operand == Some(Operand::Not) {
                        *cur = node.parent.unwrap();
                    } else {
                        if fact.determined.get() == true {
                            graph.insert_rhs(*cur, token)?;
                        } else {
                            let sub_head = graph.insert_rhs(*cur, token)?;
                            checker::infinite_rule_loop(&graph, sub_head, fact)?;
                            graph = generate_tree(graph, fact, sub_head, rules)?;
                        }
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
                "Tree builder: no current node to push fact",
            ))
        }
    }
}

fn generate_tree<'a>(
    mut graph: Graph<Token<'a>>,
    queried: &Fact,
    mut cur: NodeIndex,
    rules: &'a Rules,
) -> Result<Graph<Token<'a>>, Error> {
    for rule in rules.iter() {
        if rule.implies_fact(queried) {
            for token in rule.lhs.iter() {
                let token = *token;
                if token.is_operand() {
                    graph = push_operand_rec(graph, token, &mut cur)?;
                } else if let Some(fact) = token.fact {
                    graph = push_fact_rec(graph, rules, token, fact, &mut cur)?;
                } else {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Tree builder: empty Token",
                    ));
                }
            }
        }
    }
    Ok(graph)
}

fn get_plain_solved_queries(queries: Vec<&Fact>) -> Vec<Fact> {
    let mut solved_queries = Vec::new();
    for fact in queries.iter() {
        solved_queries.push(fact.copy());
    }
    solved_queries
}

pub fn solve(queries: Vec<&Fact>, rules: Rules) -> Result<Vec<Fact>, Error> {
    for fact in queries.iter() {
        let mut graph: Graph<Token> = Graph::new();
        let root: NodeIndex = graph.add_query(Token::new_fact(&fact));
        graph = generate_tree(graph, fact, root, &rules)?;
        // println!("{:#?}", graph);
        print_tree_to_file(&graph);
    }
    // checker::solved_queries(&facts)?;
    Ok(get_plain_solved_queries(queries))
}
