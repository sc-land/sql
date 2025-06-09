use pest::{iterators::Pair, Parser};
use serde::{Deserialize, Serialize};
use crate::sql::parser::parser::{Rule, SQLParser};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Fauna {
    Int,
    Text,
    Bool,
}

impl Fauna {

    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::fauna);

        match pair.as_str() {
            "INT" => Fauna::Int,
            "TEXT" => Fauna::Text,
            "BOOLEAN" => Fauna::Bool,
            _ => panic!("Invalid sql type: {}", pair.as_str()),
        }
    }


    pub fn from_input(input: String) -> Self {
        let pair = SQLParser::parse(Rule::fauna, &input)
            .expect("Failed to parse input")
            .next()
            .expect("No pair found");
        Fauna::from_pair(pair)
    }
}

#[cfg(test)]
mod tests {
    use crate::sql::ast::ddl::fauna::Fauna;

    #[test]
    fn test_from_str() {
        assert_eq!(Fauna::from_input("INT".to_string()), Fauna::Int);
        assert_eq!(Fauna::from_input("TEXT".to_string()), Fauna::Text);
        assert_eq!(Fauna::from_input("BOOLEAN".to_string()), Fauna::Bool);
    }
}
