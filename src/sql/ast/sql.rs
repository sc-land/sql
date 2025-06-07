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

    /// Processa múltiplas instruções SQL de um arquivo
    pub fn parse(pair: Pair<Rule>) -> Vec<Self> {
        assert_eq!(pair.as_rule(), Rule::sql);

        let mut statements = Vec::new();

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::DDL => {
                    let ddl = DDL::from_pair(inner_pair);
                    statements.push(Sql::DDL(ddl));
                }
                Rule::DML => {
                    let dml = DML::from_pair(inner_pair);
                    statements.push(Sql::DML(dml));
                }
                Rule::DQL => {
                    let dql = DQL::from_pair(inner_pair);
                    statements.push(Sql::DQL(dql));
                }
                Rule::comment => {
                    let comment = Comment::from_pair(inner_pair);
                    statements.push(Sql::Comment(comment));
                }
                _ => {
                    // Pula regras que não são SQL statements
                    continue;
                }
            }
        }

        statements
    }
}
