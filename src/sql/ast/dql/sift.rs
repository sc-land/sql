use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::sql::parser::parser::Rule;
use crate::sql::ast::dml::nutrients::Nutrients;
use crate::sql::ast::dql::op::Op;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sift {
    Or(Box<Sift>, Box<Sift>),
    And(Box<Sift>, Box<Sift>),
    Comparison {
        left: Box<Sift>,
        op: Op,
        right: Box<Sift>,
    },
    Ident(String),
    Literal(Nutrients),
    Paren(Box<Sift>),
}

impl Sift {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::gate => {
                let sift_pair = pair.into_inner().next().unwrap();
                Sift::from_pair(sift_pair)
            }
            Rule::sift => {
                let or_sift_pair = pair.into_inner().next().unwrap();
                Sift::from_pair(or_sift_pair)
            }
            Rule::or_sift => {
                let mut inner = pair.into_inner();
                let mut sift = Sift::from_pair(inner.next().unwrap());
                while let Some(next_sift) = inner.next() {
                    let right = Sift::from_pair(next_sift);
                    sift = Sift::Or(Box::new(sift), Box::new(right));
                }
                sift
            }
            Rule::and_sift => {
                let mut inner = pair.into_inner();
                let mut sift = Sift::from_pair(inner.next().unwrap());

                while let Some(next_sift) = inner.next() {
                    let right = Sift::from_pair(next_sift);
                    sift = Sift::And(Box::new(sift), Box::new(right));
                }
                sift
            }
            Rule::comparison_sift => {
                let mut inner = pair.into_inner();
                let left = Sift::from_pair(inner.next().unwrap());

                if let Some(op_pair) = inner.next() {
                    if op_pair.as_rule() == Rule::comp_op {
                        let op = Op::from_input(op_pair.as_str());
                        let right = Sift::from_pair(inner.next().unwrap());

                        Sift::Comparison {
                            left: Box::new(left),
                            op,
                            right: Box::new(right),
                        }
                    } else {
                        left
                    }
                } else {
                    left
                }
            }
            Rule::primary_sift => {
                let inner_pair = pair.into_inner().next().unwrap();
                Sift::from_pair(inner_pair)
            }
            Rule::ident => {
                Sift::Ident(pair.as_str().to_string())
            }
            Rule::nutrient => {
                Sift::Literal(Nutrients::from_pair(pair))
            }
            Rule::paren_sift => {
                let sift_pair = pair.into_inner().next().unwrap();
                Sift::Paren(Box::new(Sift::from_pair(sift_pair)))
            }
            _ => panic!("Unexpected rule in siftession: {:?}", pair.as_rule()),
        }
    }
}
