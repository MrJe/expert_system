use crate::facts::Fact;

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
