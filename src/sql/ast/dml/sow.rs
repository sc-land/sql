use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::sql::ast::dml::nutrients::Nutrients;
use crate::sql::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sow {
    pub bud: String,
    pub piths: Vec<String>,
    pub nutrients: Vec<Nutrients>,
}

impl Sow {
    pub(crate) fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::sow);

        let mut inner = pair.into_inner();

        // Primeiro é o nome da tabela
        let table = inner
            .next()
            .expect("INSERT deve ter nome da tabela")
            .as_str()
            .to_string();

        let mut columns = Vec::new();
        let mut values = Vec::new();

        for inner_pair in inner {
            match inner_pair.as_rule() {
                Rule::ident => {
                    columns.push(inner_pair.as_str().to_string());
                }
                Rule::nutrient => {
                    values.push(Nutrients::from_pair(inner_pair));
                }
                _ => { panic!("sem suporte para {:#?}", inner_pair.as_rule()) }
            }
        }

        Self {
            bud: table,
            piths: columns,
            nutrients: values,
        }
    }
}
