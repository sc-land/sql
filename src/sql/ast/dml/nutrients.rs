use serde::{Deserialize, Serialize};
use pest::iterators::Pair;
use crate::sql::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Nutrients {
    Int(i64),
    Str(String),
    Bool(bool),
}

impl Nutrients {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::nutrient);

        let inner_pair = pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::number => {
                let num = inner_pair.as_str().parse::<i64>()
                    .expect("Número deve ser válido");
                Nutrients::Int(num)
            }
            Rule::string => {
                // Remove as aspas do início e fim
                let str_content = inner_pair.as_str();
                let trimmed = &str_content[1..str_content.len()-1];
                Nutrients::Str(trimmed.to_string())
            }
            Rule::boolean => {
                let bool_val = match inner_pair.as_str().to_uppercase().as_str() {
                    "TRUE" => true,
                    "FALSE" => false,
                    _ => panic!("Boolean inválido: {}", inner_pair.as_str()),
                };
                Nutrients::Bool(bool_val)
            }
            _ => panic!("Literal tipo inesperado: {:?}", inner_pair.as_rule()),
        }
    }
}
