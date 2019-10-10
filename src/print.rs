use crate::facts::Fact;
use crate::graph::{Graph, NodeIndex};
use crate::rules::rule::token::Token;

use std::fs::File;
use std::io::{prelude::*, Error};

pub fn results(solved_queries: &[Fact]) {
    // println!("Everything worked as expected, here are the results:");
    for fact in solved_queries.iter() {
        print!("{}", fact.letter);
        if fact.state.get() {
            println!(" = True");
        } else {
            println!(" = False");
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
                fcontents.push_str(" = True\n");
            } else {
                fcontents.push_str(" = False\n");
            }
        }
    }
    f.write_all(fcontents.as_bytes())?;
    println!(
        "The output result has been printed in the following file : {}",
        fname
    );
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
            if let Some(lhs) = node.lhs {
                print_tree_rec(graph, lhs, spaces);
                if let Some(rhs) = node.rhs {
                    print!("{:<1$}-> ", "", spaces);
                    print_tree_rec(graph, rhs, spaces);
                }
            } else if let Some(fact) = node.content.fact {
                println!("{}", fact.state.get());
            } else {
                println!("Error: last node isn't a fact.");
            }
        }
    }
}

pub fn tree_to_file(graph: &Graph<Token>) {
    print_tree_rec(graph, 0, 2);
    println!();
}
