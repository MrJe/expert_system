use crate::facts;
use facts::Fact;

pub fn expert_system(file: &str) {
    let     lines = file.lines();
    let mut facts: [Fact; 26] = [Fact::new(); 26];

    facts::init(&mut facts);
    for s in lines {
        println!("line: {}", s);
    }
    facts::print(&facts);
    // facts[1].print();
}
