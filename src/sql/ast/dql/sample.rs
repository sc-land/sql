use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::sql::ast::dql::twigs::Twigs;
use crate::sql::parser::parser::Rule;
use crate::sql::ast::dql::sift::Sift;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sample {
    pub bud: String,
    pub piths: Twigs,
    pub gate: Option<Sift>,
}

impl Sample {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::sample);

        let mut inner = pair.into_inner();

        // Primeiro são as colunas
        let columns_pair = inner
            .next()
            .expect("SELECT deve ter colunas");

        let columns = Twigs::from_pair(columns_pair);

        // Segundo é o nome da tabela
        let table = inner
            .next()
            .expect("SELECT deve ter nome da tabela")
            .as_str()
            .to_string();

        // Terceiro é opcional: WHERE clause
        let gate = if let Some(where_pair) = inner.next() {
            match where_pair.as_rule() {
                Rule::gate => {
                    Some(Sift::from_pair(where_pair))
                }
                _ => panic!("Unexpected rule after table: {:?}", where_pair.as_rule()),
            }
        } else {
            None
        };

        Self {
            piths: columns,
            bud: table,
            gate,
        }
    }
}
