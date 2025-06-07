use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Op {
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
}


impl Op {
    pub fn from_input(s: &str) -> Self {
        match s {
            "=" => Op::Eq,
            "<>" | "!=" => Op::Neq,
            "<" => Op::Lt,
            "<=" => Op::Lte,
            ">" => Op::Gt,
            ">=" => Op::Gte,
            _ => panic!("Unknown comparison operator: {}", s),
        }
    }
}
