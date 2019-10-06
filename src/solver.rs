use crate::facts::Fact;
use crate::graph::{Graph, NodeIndex, Node};
use crate::rules::{rule::token::Token, Rules};
use crate::print::print_tree_to_file;
use crate::checker;

use std::io::{Error, ErrorKind};

fn get_plain_queries(queries: Vec<&Fact>) -> Vec<Fact> {
    let mut solved_queries = Vec::new();
    for fact in queries.iter() {
        solved_queries.push(fact.copy());
    }
    solved_queries
}


// REFACTO WIP TODO
// fn push_fact_rec<'a>(graph: &'a mut Graph<Token<'a>>, fact: &Fact, mut cur: NodeIndex, rules: &'a Rules, token: Token<'a>, node: &Node<Token<'a>>) -> Result<(), Error> {
//     if node.lhs.is_none() {
//         if fact.determined.get() == true {
//             graph.insert_lhs(cur, token)?;
//         } else {
//             let sub_head = graph.insert_lhs(cur, token)?;
//             checker::infinite_rule_loop(graph, sub_head, fact)?;
//             generate_tree(graph, fact, sub_head, rules)?;
//         }
//     } else {
//         while node.parent.is_some() {
//             if node.rhs.is_some() {
//                 cur = node.parent.unwrap();
//             } else {
//                 if fact.determined.get() == true {
//                     graph.insert_rhs(cur, token)?;
//                 } else {
//                     let sub_head = graph.insert_rhs(cur, token)?;
//                     checker::infinite_rule_loop(graph, sub_head, fact)?;
//                     generate_tree(graph, fact, sub_head, rules)?;
//                 }
//                 break;
//             }
//             node = graph.get(cur).unwrap(); // danger
//         }
//     }
//     Ok(())
// }

fn generate_tree<'a>(graph: &mut Graph<Token<'a>>, queried: &Fact, mut cur: NodeIndex, rules: &'a Rules) -> Result<(), Error> {
    for rule in rules.iter() {
        if rule.implies_fact(queried) {
            for token in rule.lhs.iter() {
                let token = token.clone();
                if token.is_operand() {
                    cur = graph.insert_operand(cur, token)?;
                } else if let Some(fact) = token.fact {
                    match graph.get(cur) {
                        // Some(mut node) => push_fact_rec(graph, fact, cur, rules, token, node)?, // node: &Node
                        Some(mut node) => {
                            if node.lhs.is_none() {
                                if fact.determined.get() == true {
                                    graph.insert_lhs(cur, token)?;
                                } else {
                                    let sub_head = graph.insert_lhs(cur, token)?;
                                    checker::infinite_rule_loop(graph, sub_head, fact)?;
                                    generate_tree(graph, fact, sub_head, rules)?;
                                }
                            } else {
                                while node.parent.is_some() {
                                    if node.rhs.is_some() {
                                        cur = node.parent.unwrap();
                                    } else {
                                        if fact.determined.get() == true {
                                            graph.insert_rhs(cur, token)?;
                                        } else {
                                            let sub_head = graph.insert_rhs(cur, token)?;
                                            checker::infinite_rule_loop(graph, sub_head, fact)?;
                                            generate_tree(graph, fact, sub_head, rules)?;
                                        }
                                        break;
                                    }
                                    node = graph.get(cur).unwrap(); // danger
                                }
                            }
                        },
                        None => return Err(Error::new(ErrorKind::NotFound, "Tree builder: no current node"))
                    }
                } else {
                    return Err(Error::new(ErrorKind::InvalidData, "Tree builder: empty Token"))
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
        // println!("{:#?}", graph);
        print_tree_to_file(&graph);
    }
    // checker::solved_queries(&facts)?;
    Ok(get_plain_queries(queries))
}
