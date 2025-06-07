use pest::Parser;
use serde::{Deserialize, Serialize};
use crate::sql::ast::sql::Sql;
use crate::sql::parser::parser::{Rule, SQLParser};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tree {
    pub sqls: Vec<Sql>,
}

impl Tree {
    pub fn parse_input(input: String) -> Self {
        let parsed = SQLParser::parse(Rule::sql, &input).unwrap();
        let pair = parsed.clone().next().unwrap();

        let statements = Sql::parse(pair);

        Self {
            sqls: statements,
        }
    }
}
