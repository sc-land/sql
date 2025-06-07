use pest::iterators::Pair;
use pest::Parser;
use serde::{Deserialize, Serialize};
use crate::sql::parser::parser::{Rule, SQLParser};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Anchor {
    Essence, // NotNull
    Nucleus, // PrimaryKey
    Axis,    // Unique
}

impl Anchor {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::anchor);

        match pair.as_str() {
            "NOT NULL" => Anchor::Essence,
            "PRIMARY KEY" => Anchor::Nucleus,
            "UNIQUE" => Anchor::Axis,
            _ => panic!("Unknown column constraint: {}", pair.as_str()),
        }
    }

    pub fn from_input(input: String) -> Self {
        let pair = SQLParser::parse(Rule::anchor, &input)
            .expect("Failed to parse input")
            .next()
            .expect("No pair found");
        Anchor::from_pair(pair)
    }
}

#[cfg(test)]
mod tests {
    use crate::sql::ast::ddl::anchor::Anchor;

    #[test]
    fn not_null_is_essence() {
        let input = "NOT NULL".to_string();
        let anchor = Anchor::from_input(input);
        assert_eq!(anchor, Anchor::Essence);
    }

    #[test]
    fn unique_is_axis() {
        let input = "UNIQUE".to_string();
        let anchor = Anchor::from_input(input);
        assert_eq!(anchor, Anchor::Axis);
    }

    #[test]
    fn primary_key_is_nucleus() {
        let input = "PRIMARY KEY".to_string();
        let anchor = Anchor::from_input(input);
        assert_eq!(anchor, Anchor::Nucleus);
    }
}
