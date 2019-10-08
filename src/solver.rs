
use std::io::{Error};

use crate::print;
use crate::facts::Fact;
use crate::tree_builder;
use crate::graph::{Graph, NodeIndex};
use crate::rules::{rule::token::Token, Rules};

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
        graph = tree_builder::generate(graph, fact, root, &rules)?;
        // println!("{:#?}", graph);
        print::tree_to_file(&graph);
    }
    // checker::solved_queries(&facts)?;
    Ok(get_plain_solved_queries(queries))
}
