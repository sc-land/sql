use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::sql::ast::ddl::anchor::Anchor;
use crate::sql::ast::ddl::fauna::Fauna;
use crate::sql::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stem {
    pub vein: String,
    pub fauna: Fauna,
    pub anchors: Vec<Anchor>,
}

impl Stem {
    pub fn from_pair(pair: Pair<Rule>) -> Stem {
        assert_eq!(pair.as_rule(), Rule::stem);

        let mut inner = pair.into_inner();
        let vein = inner
            .next()
            .expect("column_def deve ter um nome")
            .as_str()
            .to_string();
        
        let fauna = Fauna::from_pair(inner
            .next()
            .expect("column_def deve ter um tipo SQL"));
        
        let mut anchors = Vec::new();
        for constraint_pair in inner {
            match constraint_pair.as_rule() {
                Rule::anchor => {
                    let constraint = Anchor::from_pair(constraint_pair);
                    anchors.push(constraint);
                }
                _ => panic!("Unexpected rule in column_def: {:?}", constraint_pair.as_rule()),
            }
        }

        Self {
            vein,
            fauna,
            anchors,
        }
    }
}
