pub mod sql;

pub use sql::*;

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::sql::parser::tree::Tree;

    #[test]
    fn it_works() {
        let sql = fs::read_to_string("eg/ddl/create.sql").unwrap();
        let tree = Tree::parse_input(sql);

        for sql in tree.sqls {
            println!("Parsed SQL: {:#?}", sql);
        }
    }
}
