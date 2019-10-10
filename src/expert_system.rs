// const OUTPUT_FILE: &str = "RESULT.txt";

use crate::facts::{Fact, Facts};
use crate::options::Options;
use crate::print;
use crate::rules::Rules;
use crate::solver;

use std::fs::File;
use std::io::{prelude::*, BufReader, Error, ErrorKind};
use std::path::Path;

fn parser<'a>(file: File, facts: &'a Facts, options: &Options) -> Result<Rules<'a>, Error> {
    let reader = BufReader::new(file);
    let mut rules = Rules::new();
    if options.file {
        println!("=== FILE ===");
    }
    for line in reader.lines() {
        if let Ok(line) = line {
            if options.file {
                println!("{}", line);
            }
            match line.trim().chars().next() {
                Some('A'..='Z') | Some('(') | Some('!') => rules.set_rule(&facts, &line)?,
                Some('=') => facts.set_initial_facts(&line)?,
                Some('?') => facts.set_queries(&line)?,
                Some('#') | None => continue,
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("Input file: unexpected char (line: {})", line),
                    ))
                }
            }
        }
    }
    Ok(rules)
}

fn queries_of_parsed(facts: &Facts) -> Vec<&Fact> {
    let mut queries = Vec::new();
    for fact in facts.fact_arr.iter() {
        if fact.queried.get() {
            queries.push(fact);
        }
    }
    queries
}

fn expert_system(file: File, options: &Options) -> Result<Vec<Fact>, Error> {
    let facts = Facts::new();
    let mut rules = parser(file, &facts, options)?;
    rules.as_reverse_polish_notation()?;
    let queries = queries_of_parsed(&facts);
    if queries.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "No queries provided, tho nothing to solve",
        ));
    }
    let solved_queries: Vec<Fact> = solver::solve(queries, rules, options)?;
    Ok(solved_queries)
}

fn expert_system_wrapper(file: File, options: &Options) {
    match expert_system(file, options) {
        Ok(solved_queries) => {
            // print::solved_to_file(OUTPUT_FILE, &solved_queries);
            print::results(&solved_queries);
        }
        Err(error) => eprintln!(
            "Oops, something went wrong, shutting program down.\nError: {:?}",
            error.to_string()
        ),
    }
}

pub fn run(filename: &str, options: &Options) {
    if Path::new(filename).is_dir() {
        println!("open: {}: Is a directory", filename);
    } else {
        match File::open(filename) {
            Ok(file) => expert_system_wrapper(file, options),
            Err(error) => eprintln!("open: {}: {:?}", filename, error.to_string()),
        };
    }
}
