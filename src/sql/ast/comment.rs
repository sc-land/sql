use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::sql::parser::parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Comment {
    Line(String),
    Block(String),
}

impl Comment {
    pub(crate) fn from_pair(_p0: Pair<Rule>) -> Self {
        // Implementação temporária - apenas retorna um comentário vazio
        Comment::Line("temp comment".to_string())
    }
}
