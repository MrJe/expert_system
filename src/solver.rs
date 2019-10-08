
use std::io::{Error};

use crate::print;
use crate::facts::Fact;
use crate::tree_builder;
use crate::graph::{Graph, NodeIndex};
use crate::rules::{rule::token::{Token, Operand}, Rules};

fn get_plain_solved_queries(queries: Vec<&Fact>) -> Vec<Fact> {
    let mut solved_queries = Vec::new();
    for fact in queries.iter() {
        solved_queries.push(fact.copy());
    }
    solved_queries
}

fn resolve_tree(graph: &Graph<Token>, cur: NodeIndex) -> bool {
    match graph.get(cur) {
        None => panic!("Error: print_tree_rec() out of bounds."),
        Some(node) => {
            let c = node.content;
            if c.is_fact() {
                if node.lhs.is_some() == false {
                    return c.get_state()
                }
                return resolve_tree(graph, node.lhs.unwrap())
            } else {
                let lhs = node.lhs.unwrap();
                if c.operand.unwrap() != Operand::Not {
                    let rhs = node.rhs.unwrap();
                    return c.resolve_op(resolve_tree(graph, lhs),
                                        resolve_tree(graph, rhs))
                }
                return !resolve_tree(graph, lhs)
            }
        }
    }
}

pub fn solve(queries: Vec<&Fact>, rules: Rules) -> Result<Vec<Fact>, Error> {
    for fact in queries.iter() {
        let mut graph: Graph<Token> = Graph::new();
        let root: NodeIndex = graph.add_query(Token::new_fact(&fact));
        graph = tree_builder::generate(graph, fact, root, &rules)?;
        // println!("{:#?}", graph);
        let mut res = resolve_tree(&graph, 0);
        if fact.reverse_state.get() {
            res = !res;
            fact.reverse_state.set(false);
        }
        fact.state.set(res);
        print::tree_to_file(&graph);
    }
    // checker::solved_queries(&facts)?;
    Ok(get_plain_solved_queries(queries))
}
