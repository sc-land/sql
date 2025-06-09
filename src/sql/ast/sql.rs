// Main SQL AST node that encompasses all SQL types
use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::sql::{ast::{comment::Comment, ddl::DDL, dql::DQL}, parser::parser::Rule};
use crate::sql::ast::dml::DML;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sql {
    DDL(DDL),
    DML(DML),
    DQL(DQL),
    Comment(Comment),
}

impl Sql {
    pub fn from_pair(pair: Pair<Rule>) -> Vec<Self> {
        assert_eq!(pair.as_rule(), Rule::sql);

        pair.into_inner()
            .filter_map(|p| match p.as_rule() {
                Rule::DDL => Some(Sql::DDL(DDL::from_pair(p))),
                Rule::DML => Some(Sql::DML(DML::from_pair(p))),
                Rule::DQL => Some(Sql::DQL(DQL::from_pair(p))),
                Rule::comment => Some(Sql::Comment(Comment::from_pair(p))),
                _ => None,
            })
            .collect()
    }
}
