pub mod sample;
pub mod twigs;
pub mod sift;
mod op;

use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::sql::ast::dql::sample::Sample;
use crate::sql::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DQL {
    Sample(Sample),
}

impl DQL {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::DQL);

        let inner_pair = pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::sample => {
                let select = Sample::from_pair(inner_pair);
                DQL::Sample(select)
            }
            _ => panic!("Unexpected rule: {:?}", inner_pair.as_rule()),
        }
    }
}
