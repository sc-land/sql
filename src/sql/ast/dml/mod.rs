pub mod sow;
pub mod nutrients;

use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::sql::ast::dml::sow::Sow;
use crate::sql::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DML {
    Sow(Sow),
}

impl DML {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::DML);

        let inner_pair = pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::sow => { DML::Sow(Sow::from_pair(inner_pair)) }
            _ => panic!("Unexpected rule: {:?}", inner_pair.as_rule()),
        }
    }
}
