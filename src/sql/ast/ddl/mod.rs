pub mod seed;
pub mod anchor;
pub mod fauna;
pub mod stem;

use crate::sql::parser::parser::Rule;
use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::sql::ast::ddl::seed::Seed;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DDL {
    Seed(Seed),
}

impl DDL {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::DDL);

        let inner_pair = pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::seed => { DDL::Seed(Seed::from_pair(inner_pair)) }
            _ => panic!("Unexpected rule: {:?}", inner_pair.as_rule()),
        }
    }
}
