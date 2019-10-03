use crate::facts::Fact;
use crate::graph::Graph;
use crate::rules::{rule::token::Token, Rules};

use std::io::{Error};

fn get_plain_queries(queries: Vec<&Fact>) -> Vec<Fact> {
	let mut solved_queries = Vec::new();
	for fact in queries.iter() {
		solved_queries.push(fact.copy());
    }
	solved_queries
}

pub fn solve(queries: Vec<&Fact>, rules: Rules) -> Result<Vec<Fact>, Error> {
    let mut graph: Graph<Token> = Graph::new();
   	for fact in queries.iter() {
		graph.add_query(Token::new_fact(&fact));
    }
	println!("{:#?}", graph);
    // checker::solved_queries(&facts)?;
	Ok(get_plain_queries(queries))
}
