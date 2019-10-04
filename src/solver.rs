use crate::facts::Fact;
use crate::graph::{Graph, NodeIndex};
use crate::rules::{rule::token::Token, Rules};

use std::io::{Error, ErrorKind};

fn get_plain_queries(queries: Vec<&Fact>) -> Vec<Fact> {
    let mut solved_queries = Vec::new();
    for fact in queries.iter() {
        solved_queries.push(fact.copy());
    }
    solved_queries
}

fn generate_tree<'a>(graph: &mut Graph<Token<'a>>, queried: &Fact, mut cur: NodeIndex, rules: &'a Rules) -> Result<(), Error> {
    for rule in rules.iter() {
        if rule.implies_fact(queried) {
            for token in rule.lhs.iter() {
                let token = token.clone();
                if token.is_operand() {
                    cur = graph.insert_operand(cur, token)?;
                } else {
                    match graph.get_mut(cur) {
                        Some(node) => { // node
                            if node.lhs.is_none() {
                                graph.insert_lhs(cur, token)?;
                            } else {
                                while node.parent.is_some() {
                                    if node.rhs.is_some() {
                                        cur = node.parent.unwrap();
                                    } else {
                                        cur = graph.insert_rhs(cur, token)?;
                                        break;
                                    }
                                    // node = node.parent;
                                }
                            }
                        },
                        None => return Err(Error::new(ErrorKind::NotFound, "no cur node"))
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn solve(queries: Vec<&Fact>, rules: Rules) -> Result<Vec<Fact>, Error> {
    for fact in queries.iter() {
        let mut graph: Graph<Token> = Graph::new();
        let root: NodeIndex = graph.add_query(Token::new_fact(&fact));
        generate_tree(&mut graph, fact, root, &rules)?;
        println!("{:#?}", graph);
    }
    // checker::solved_queries(&facts)?;
    Ok(get_plain_queries(queries))
}
