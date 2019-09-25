use super::token::{Operand, Token};

pub fn apply_on_vec<'rule>(_rule: &Vec<Token<'rule>>) -> Vec<Token<'rule>> {
    println!("RuleRPN");
    vec![Token::new(None, None)]
}
