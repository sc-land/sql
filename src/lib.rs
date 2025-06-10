pub mod sql;

pub use sql::*;

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::ast::ddl::DDL;
    use crate::ast::sql::Sql;
    use crate::sql::parser::tree::Tree;

    #[test]
    fn it_works() {
        let sql = fs::read_to_string("eg/ddl/create.sql").unwrap();
        assert!(sql.contains("CREATE TABLE Enzyme"), "SQL should contain 'CREATE TABLE Enzyme'");
        assert!(sql.contains("CREATE TABLE Genome"), "SQL should contain 'CREATE TABLE Genome'");
        assert!(sql.contains("CREATE TABLE Bug"), "SQL should contain 'CREATE TABLE Bug'");
        let tree = Tree::parse(sql);

        assert_eq!(tree.sqls.len(), 3, "Expected exactly 3 SQL statements");

        // Validação completa da tabela Enzyme
        match &tree.sqls[0] {
            Sql::DDL(DDL::Seed(seed)) => {
                assert_eq!(seed.bud, "Enzyme");
                assert_eq!(seed.stems.len(), 5, "Enzyme should have 5 columns");

                // registry_number INT PRIMARY KEY
                let registry_col = &seed.stems[0];
                assert_eq!(registry_col.vein, "registry_number");
                assert_eq!(registry_col.fauna, crate::ast::ddl::fauna::Fauna::Int);
                assert_eq!(registry_col.anchors.len(), 1);
                assert_eq!(registry_col.anchors[0], crate::ast::ddl::anchor::Anchor::Nucleus);

                // substrate TEXT
                let substrate_col = &seed.stems[1];
                assert_eq!(substrate_col.vein, "substrate");
                assert_eq!(substrate_col.fauna, crate::ast::ddl::fauna::Fauna::Text);
                assert_eq!(substrate_col.anchors.len(), 0);

                // optimal_ph INT
                let ph_col = &seed.stems[2];
                assert_eq!(ph_col.vein, "optimal_ph");
                assert_eq!(ph_col.fauna, crate::ast::ddl::fauna::Fauna::Int);
                assert_eq!(ph_col.anchors.len(), 0);

                // temperature_sensitive BOOLEAN
                let temp_col = &seed.stems[3];
                assert_eq!(temp_col.vein, "temperature_sensitive");
                assert_eq!(temp_col.fauna, crate::ast::ddl::fauna::Fauna::Bool);
                assert_eq!(temp_col.anchors.len(), 0);

                // active_site_count INT
                let site_col = &seed.stems[4];
                assert_eq!(site_col.vein, "active_site_count");
                assert_eq!(site_col.fauna, crate::ast::ddl::fauna::Fauna::Int);
                assert_eq!(site_col.anchors.len(), 0);
            }
            _ => panic!("Expected DDL Seed statement for Enzyme"),
        }

        // Validação completa da tabela Genome
        match &tree.sqls[1] {
            Sql::DDL(DDL::Seed(seed)) => {
                assert_eq!(seed.bud, "Genome");
                assert_eq!(seed.stems.len(), 4, "Genome should have 4 columns");

                // accession_code INT PRIMARY KEY
                let accession_col = &seed.stems[0];
                assert_eq!(accession_col.vein, "accession_code");
                assert_eq!(accession_col.fauna, crate::ast::ddl::fauna::Fauna::Int);
                assert_eq!(accession_col.anchors.len(), 1);
                assert_eq!(accession_col.anchors[0], crate::ast::ddl::anchor::Anchor::Nucleus);

                // chromosome_count INT
                let chromosome_col = &seed.stems[1];
                assert_eq!(chromosome_col.vein, "chromosome_count");
                assert_eq!(chromosome_col.fauna, crate::ast::ddl::fauna::Fauna::Int);
                assert_eq!(chromosome_col.anchors.len(), 0);

                // gc_content INT
                let gc_col = &seed.stems[2];
                assert_eq!(gc_col.vein, "gc_content");
                assert_eq!(gc_col.fauna, crate::ast::ddl::fauna::Fauna::Int);
                assert_eq!(gc_col.anchors.len(), 0);

                // is_circular BOOLEAN
                let circular_col = &seed.stems[3];
                assert_eq!(circular_col.vein, "is_circular");
                assert_eq!(circular_col.fauna, crate::ast::ddl::fauna::Fauna::Bool);
                assert_eq!(circular_col.anchors.len(), 0);
            }
            _ => panic!("Expected DDL Seed statement for Genome"),
        }

        // Validação completa da tabela Bug
        match &tree.sqls[2] {
            Sql::DDL(DDL::Seed(seed)) => {
                assert_eq!(seed.bud, "Bug");
                assert_eq!(seed.stems.len(), 6, "Bug should have 6 columns");

                // specimen_tag INT PRIMARY KEY
                let tag_col = &seed.stems[0];
                assert_eq!(tag_col.vein, "specimen_tag");
                assert_eq!(tag_col.fauna, crate::ast::ddl::fauna::Fauna::Int);
                assert_eq!(tag_col.anchors.len(), 1);
                assert_eq!(tag_col.anchors[0], crate::ast::ddl::anchor::Anchor::Nucleus);

                // wing_span INT
                let wing_col = &seed.stems[1];
                assert_eq!(wing_col.vein, "wing_span");
                assert_eq!(wing_col.fauna, crate::ast::ddl::fauna::Fauna::Int);
                assert_eq!(wing_col.anchors.len(), 0);

                // colony_role TEXT
                let role_col = &seed.stems[2];
                assert_eq!(role_col.vein, "colony_role");
                assert_eq!(role_col.fauna, crate::ast::ddl::fauna::Fauna::Text);
                assert_eq!(role_col.anchors.len(), 0);

                // venomous BOOLEAN
                let venom_col = &seed.stems[3];
                assert_eq!(venom_col.vein, "venomous");
                assert_eq!(venom_col.fauna, crate::ast::ddl::fauna::Fauna::Bool);
                assert_eq!(venom_col.anchors.len(), 0);

                // antenna_length INT
                let antenna_col = &seed.stems[4];
                assert_eq!(antenna_col.vein, "antenna_length");
                assert_eq!(antenna_col.fauna, crate::ast::ddl::fauna::Fauna::Int);
                assert_eq!(antenna_col.anchors.len(), 0);

                // metamorphosis_complete BOOLEAN
                let metamorphosis_col = &seed.stems[5];
                assert_eq!(metamorphosis_col.vein, "metamorphosis_complete");
                assert_eq!(metamorphosis_col.fauna, crate::ast::ddl::fauna::Fauna::Bool);
                assert_eq!(metamorphosis_col.anchors.len(), 0);
            }
            _ => panic!("Expected DDL Seed statement for Bug"),
        }
    }

    #[test]
    fn test_table_structures_detailed() {
        let sql = fs::read_to_string("eg/ddl/create.sql").unwrap();
        let tree = Tree::parse(sql);

        println!("\n=== VALIDAÇÃO DETALHADA DAS TABELAS ===\n");

        // Estrutura esperada das tabelas
        let expected_structures = [
            ("Enzyme", vec![
                ("registry_number", "Int", true),   // PRIMARY KEY
                ("substrate", "Text", false),
                ("optimal_ph", "Int", false),
                ("temperature_sensitive", "Bool", false),
                ("active_site_count", "Int", false),
            ]),
            ("Genome", vec![
                ("accession_code", "Int", true),    // PRIMARY KEY
                ("chromosome_count", "Int", false),
                ("gc_content", "Int", false),
                ("is_circular", "Bool", false),
            ]),
            ("Bug", vec![
                ("specimen_tag", "Int", true),      // PRIMARY KEY
                ("wing_span", "Int", false),
                ("colony_role", "Text", false),
                ("venomous", "Bool", false),
                ("antenna_length", "Int", false),
                ("metamorphosis_complete", "Bool", false),
            ]),
        ];

        for (table_index, (expected_table_name, expected_columns)) in expected_structures.iter().enumerate() {
            match &tree.sqls[table_index] {
                Sql::DDL(DDL::Seed(seed)) => {
                    println!("📋 Tabela: {}", seed.bud);
                    assert_eq!(seed.bud, *expected_table_name);
                    assert_eq!(seed.stems.len(), expected_columns.len(),
                              "Tabela {} deveria ter {} colunas", expected_table_name, expected_columns.len());

                    for (col_index, (expected_name, expected_type, has_primary_key)) in expected_columns.iter().enumerate() {
                        let column = &seed.stems[col_index];

                        println!("  └─ {}: {} {}",
                                column.vein,
                                format!("{:?}", column.fauna),
                                if *has_primary_key { "(PRIMARY KEY)" } else { "" });

                        // Validar nome da coluna
                        assert_eq!(column.vein, *expected_name,
                                  "Coluna {} da tabela {}", col_index, expected_table_name);

                        // Validar tipo da coluna
                        let expected_fauna = match *expected_type {
                            "Int" => crate::ast::ddl::fauna::Fauna::Int,
                            "Text" => crate::ast::ddl::fauna::Fauna::Text,
                            "Bool" => crate::ast::ddl::fauna::Fauna::Bool,
                            _ => panic!("Tipo não esperado: {}", expected_type),
                        };
                        assert_eq!(column.fauna, expected_fauna,
                                  "Tipo da coluna {} da tabela {}", expected_name, expected_table_name);

                        // Validar constraints
                        if *has_primary_key {
                            assert_eq!(column.anchors.len(), 1,
                                      "Coluna {} deveria ter PRIMARY KEY", expected_name);
                            assert_eq!(column.anchors[0], crate::ast::ddl::anchor::Anchor::Nucleus,
                                      "Coluna {} deveria ser PRIMARY KEY", expected_name);
                        } else {
                            assert_eq!(column.anchors.len(), 0,
                                      "Coluna {} não deveria ter constraints", expected_name);
                        }
                    }
                    println!();
                }
                _ => panic!("Esperado DDL Seed para tabela {}", expected_table_name),
            }
        }

        println!("✅ Todas as validações passaram!");
    }
}
