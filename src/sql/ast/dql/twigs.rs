use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::sql::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Twigs {
    All,
    Named(Vec<String>),
}

impl Twigs {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::twigs);

        let content = pair.as_str();

        if content == "*" {
            Twigs::All
        } else {
            // É uma lista de colunas nomeadas
            let mut columns = Vec::new();

            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() == Rule::ident {
                    columns.push(inner_pair.as_str().to_string());
                }
            }

            Twigs::Named(columns)
        }
    }
}
