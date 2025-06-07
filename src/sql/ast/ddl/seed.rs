use pest::iterators::Pair;
use pest::Parser;
use serde::{Deserialize, Serialize};
use crate::sql::ast::ddl::stem::Stem;
use crate::sql::parser::parser::{Rule, SQLParser};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Seed {
    pub bud: String,
    pub stems: Vec<Stem>,
}

impl Seed {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::seed);

        let mut inner = pair.into_inner();
        let mut stems: Vec<Stem> = Vec::new();

        let bud = inner
            .next()
            .expect("CREATE TABLE deve ter um nome")
            .as_str()
            .to_string();

        for inner_pair in inner {
            match inner_pair.as_rule() {
                Rule::stem => {
                    stems.push(Stem::from_pair(inner_pair));
                }
                _ => panic!("Unexpected rule: {:?}", inner_pair.as_rule()),
            }
        }

        Self { bud, stems }
    }

    pub fn from_input(input: String) -> Self {
        let pair = SQLParser::parse(Rule::seed, &input)
            .expect("Failed to parse input")
            .next()
            .expect("No pair found");
        Seed::from_pair(pair)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::sql::ast::ddl::seed::Seed;
    use crate::sql::ast::ddl::fauna::Fauna;
    use crate::sql::ast::ddl::anchor::Anchor;

    #[test]
    fn test_seed_complete(){
        let sql = fs::read_to_string("eg/ddl/create.sql").unwrap();
        let seed = Seed::from_input(sql);

        // Testa o nome da tabela (bud)
        assert_eq!(seed.bud, "Product");

        // Testa que temos 4 stems (colunas)
        assert_eq!(seed.stems.len(), 4);

        // Testa primeira coluna: id INT PRIMARY KEY
        let id_stem = &seed.stems[0];
        assert_eq!(id_stem.vein, "id");
        assert_eq!(id_stem.fauna, Fauna::Int);
        assert_eq!(id_stem.anchors.len(), 1);
        assert_eq!(id_stem.anchors[0], Anchor::Nucleus); // PRIMARY KEY

        // Testa segunda coluna: title TEXT
        let title_stem = &seed.stems[1];
        assert_eq!(title_stem.vein, "title");
        assert_eq!(title_stem.fauna, Fauna::Text);
        assert_eq!(title_stem.anchors.len(), 0); // Sem constraints

        // Testa terceira coluna: price INT
        let price_stem = &seed.stems[2];
        assert_eq!(price_stem.vein, "price");
        assert_eq!(price_stem.fauna, Fauna::Int);
        assert_eq!(price_stem.anchors.len(), 0); // Sem constraints

        // Testa quarta coluna: available BOOLEAN
        let available_stem = &seed.stems[3];
        assert_eq!(available_stem.vein, "available");
        assert_eq!(available_stem.fauna, Fauna::Bool);
        assert_eq!(available_stem.anchors.len(), 0); // Sem constraints
    }
}
