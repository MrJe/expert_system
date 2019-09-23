use super::fact::Fact;

pub fn print(facts: &[Fact]) {
    for f in facts {
        f.print();
    }
}
