use super::fact::Fact;

pub fn init(facts: &mut [Fact]) {
    for (i, f) in facts.iter_mut().enumerate() {
        f.letter = (65 + i as u8) as char;
    }
}
