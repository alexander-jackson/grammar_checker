use std::convert::TryFrom;

use crate::Production;

#[derive(Debug)]
pub struct Rule<'a> {
    pub non_terminal: &'a str,
    pub derivations: Vec<Production<'a>>,
}

impl<'a> Rule<'a> {
    pub fn new(non_terminal: &'a str, derivations: Vec<Production<'a>>) -> Self {
        Self {
            non_terminal,
            derivations,
        }
    }
}

impl<'a> TryFrom<&'a str> for Rule<'a> {
    type Error = &'static str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.split(" ::= ").collect();

        let prods: Vec<Production> = split[1]
            .split(" | ")
            .map(|x| Production::try_from(x).unwrap())
            .collect();

        Ok(Rule::new(split[0], prods))
    }
}
