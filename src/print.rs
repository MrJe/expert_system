use crate::facts::Fact;
use crate::graph::{Graph, NodeIndex};
use crate::rules::rule::token::Token;

use std::fs::File;
use std::io::{prelude::*, Error};

pub fn results(solved_queries: &[Fact]) {
    println!("Everything worked as expected, here are the results:");
    for fact in solved_queries.iter() {
        print!("{}", fact.letter);
        if fact.state.get() {
            println!(" = TRUE");
        } else {
            println!(" = FALSE");
        }
    }
}

pub fn solved_to_file(fname: &str, solved_queries: &[Fact]) -> Result<(), Error> {
    let mut f = File::create(fname)?;
    let mut fcontents = String::new();
    for fact in solved_queries.iter() {
        if fact.queried.get() {
            fcontents.push(fact.letter);
            if fact.state.get() {
                fcontents.push_str(" = TRUE\n");
            } else {
                fcontents.push_str(" = FALSE\n");
            }
        }
    }
    f.write_all(fcontents.as_bytes())?;
    // println!(
    //     "The output result has been printed in the following file : {}",
    //     fname
    // );
    Ok(())
}

fn print_tree_rec(graph: &Graph<Token>, cur: NodeIndex, mut spaces: usize) {
    match graph.get(cur) {
        None => println!("Error: print_tree_rec() out of bounds."),
        Some(node) => {
            let t_char = node.content.get_token_char();
            print!("{} -> ", t_char);
            if cur > 0 {
                spaces += 5;
            }
            if node.lhs.is_some() {
                print_tree_rec(graph, node.lhs.unwrap(), spaces);
                if node.rhs.is_some() {
                    print!("{:<1$}-> ", "", spaces);
                    print_tree_rec(graph, node.rhs.unwrap(), spaces);
                }
            } else if node.content.is_fact() {
                println!("{}", node.content.get_state());
            } else {
                println!("Error: last node isn't a fact.");
            }
        }
    }
}

pub fn tree_to_file(graph: &Graph<Token>) {
    print_tree_rec(graph, 0, 2);
}
