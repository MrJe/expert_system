use crate::facts::Fact;
use crate::graph::{Graph, NodeIndex};
use crate::rules::{rule::token::Token};

use std::fs::File;
use std::io::{prelude::*, Error};

pub fn results(solved_queries: &Vec<Fact>) {
    println!("Everything worked as expected, here are the results:");
    for fact in solved_queries.iter() {
        print!("{}", fact.letter);
        match fact.state.get() {
            true => println!(" = TRUE"),
            false => println!(" = FALSE"),
        }
    }
}

pub fn solved_to_file(fname: &str, solved_queries: &Vec<Fact>) -> Result<(), Error> {
    let mut f = File::create(fname)?;
    let mut fcontents = String::new();
    for fact in solved_queries.iter() {
        if fact.queried.get() == true {
            fcontents.push(fact.letter);
            match fact.state.get() {
                true => fcontents.push_str(" = TRUE\n"),
                false => fcontents.push_str(" = FALSE\n"),
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

pub fn print_tree_to_file(graph: &Graph<Token>) {
    let mut spaces = 0;
    let mut cur: NodeIndex = 0;
    loop {
        match graph.get(cur) {
            Some(mut node)  => {
                let t_char = node.content.get_token_char();
                print!("{} -> ", t_char);
                if node.lhs.is_some() {
                    cur = node.lhs.unwrap();
                    spaces += 5;
                } else {
                    println!("{}", node.content.get_state());
                    while node.parent.is_some() {
                        node = graph.get(node.parent.unwrap()).unwrap();
                        spaces -= 3;
                        let mut t_char_c = '@';
                        if node.rhs.is_some() {
                            let rhs_id = node.rhs.unwrap();
                            t_char_c = graph.get(rhs_id).unwrap().content.get_token_char();
                            if t_char_c != t_char {
                                cur = node.rhs.unwrap();
                                print!("{:<1$}-> ", "", spaces);
                                break;
                            }
                        }
                        if t_char_c == t_char {
                            spaces += 1;
                        }
                    }
                    if node.parent.is_some() == false {
                        return ;
                    }
                }
            },
            None            => break,
        }
    }
}
