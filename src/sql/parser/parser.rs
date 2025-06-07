use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sql/sql.pest"]
pub struct SQLParser;
