# SQL Parser & AST

🌱 A Rust-based SQL parser that transforms SQL statements into an Abstract Syntax Tree (AST) using nature-inspired terminology.

## 🌿 Project Overview

This project provides a comprehensive SQL parser built with Rust and Pest, featuring a unique botanical metaphor for SQL constructs:

- **Seed** → CREATE TABLE statements
- **Stem** → Table columns
- **Bud** → Table name
- **Vein** → Column name
- **Fauna** → Data types (INT, TEXT, BOOLEAN)
- **Anchor** → Constraints (PRIMARY KEY, NOT NULL, UNIQUE)

## ✨ Features

- 🔍 **Complete SQL Parsing**: DDL, DML, DQL, and Comments
- 🌲 **Botanical AST**: Nature-inspired naming for intuitive code understanding
- ⚡ **Functional Design**: Optimized with Rust iterators and functional patterns
- 🧪 **Comprehensive Testing**: Full validation of table structures and constraints
- 📦 **Serde Support**: JSON serialization/deserialization of AST nodes
- 🎯 **Type Safety**: Leverages Rust's type system for parse guarantees

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ (using 2024 edition)
- Cargo

### Installation

```bash
git clone <repository-url>
cd sql
cargo build
```

### Running Tests

```bash
cargo test
```

For detailed test output:
```bash
cargo test -- --nocapture
```

## 📖 Usage

### Basic Parsing

```rust
use sql::sql::parser::tree::Tree;

let sql_input = r#"
CREATE TABLE Product (
    id INT PRIMARY KEY,
    title TEXT,
    price INT,
    available BOOLEAN
);
"#;

let tree = Tree::parse(sql_input.to_string());
println!("{:#?}", tree);
```

### Working with AST

```rust
use sql::ast::{sql::Sql, ddl::DDL};

// Parse multiple SQL statements
let tree = Tree::parse(sql_input.to_string());

for sql in tree.sqls {
    match sql {
        Sql::DDL(DDL::Seed(seed)) => {
            println!("Table: {}", seed.bud);

            for stem in seed.stems {
                println!("  Column: {} ({:?})", stem.vein, stem.fauna);

                for anchor in stem.anchors {
                    println!("    Constraint: {:?}", anchor);
                }
            }
        }
        _ => println!("Other SQL statement type"),
    }
}
```

## 🏗️ Architecture

### Project Structure

```
src/
├── lib.rs                 # Main library entry point
└── sql/
    ├── mod.rs             # SQL module exports
    ├── sql.pest           # Pest grammar definition
    ├── ast/               # Abstract Syntax Tree nodes
    │   ├── sql.rs         # Root SQL enum
    │   ├── ddl/           # Data Definition Language
    │   │   ├── mod.rs     # DDL exports
    │   │   ├── seed.rs    # CREATE TABLE (Seed)
    │   │   ├── stem.rs    # Table columns (Stem)
    │   │   ├── fauna.rs   # Data types (Fauna)
    │   │   └── anchor.rs  # Constraints (Anchor)
    │   ├── dml/           # Data Manipulation Language
    │   └── dql/           # Data Query Language
    └── parser/            # Parser implementation
        ├── parser.rs      # Pest parser
        └── tree.rs        # Parse tree wrapper
```

### AST Hierarchy

```
Tree
└── sqls: Vec<Sql>
    ├── DDL(DDL)
    │   └── Seed(Seed)        # CREATE TABLE
    │       ├── bud: String   # Table name
    │       └── stems: Vec<Stem>  # Columns
    │           ├── vein: String      # Column name
    │           ├── fauna: Fauna      # Data type
    │           └── anchors: Vec<Anchor>  # Constraints
    ├── DML(DML)
    ├── DQL(DQL)
    └── Comment(Comment)
```

## 🧪 Testing

The project includes comprehensive tests covering:

### Table Structure Validation
- ✅ 3 complete test tables (Enzyme, Genome, Bug)
- ✅ All column names, types, and constraints
- ✅ PRIMARY KEY validation
- ✅ Table structure integrity

### Example Test Output
```
=== VALIDAÇÃO DETALHADA DAS TABELAS ===

📋 Tabela: Enzyme
  └─ registry_number: Int (PRIMARY KEY)
  └─ substrate: Text
  └─ optimal_ph: Int
  └─ temperature_sensitive: Bool
  └─ active_site_count: Int

📋 Tabela: Genome
  └─ accession_code: Int (PRIMARY KEY)
  └─ chromosome_count: Int
  └─ gc_content: Int
  └─ is_circular: Bool

📋 Tabela: Bug
  └─ specimen_tag: Int (PRIMARY KEY)
  └─ wing_span: Int
  └─ colony_role: Text
  └─ venomous: Bool
  └─ antenna_length: Int
  └─ metamorphosis_complete: Bool

✅ Todas as validações passaram!
```

## 📊 Supported SQL Features

### Data Definition Language (DDL)
- [x] `CREATE TABLE` statements
- [x] Column definitions with data types
- [x] Primary key constraints
- [x] Multiple table support

### Data Types (Fauna)
- [x] `INT` → `Fauna::Int`
- [x] `TEXT` → `Fauna::Text`
- [x] `BOOLEAN` → `Fauna::Bool`

### Constraints (Anchors)
- [x] `PRIMARY KEY` → `Anchor::Nucleus`
- [x] `NOT NULL` → `Anchor::Essence`
- [x] `UNIQUE` → `Anchor::Axis`

### Planned Features
- [ ] Data Manipulation Language (DML)
- [ ] Data Query Language (DQL)
- [ ] More data types (DATE, DECIMAL, etc.)
- [ ] Foreign key constraints
- [ ] Complex expressions

## 🔧 Development

### Dependencies

```toml
[dependencies]
pest = "2.7"
pest_derive = "2.7"
thiserror = "2.0.12"
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0"
```

### Code Style

The project follows functional programming patterns where possible:

```rust
// Functional approach with iterators
pair.into_inner()
    .filter_map(|p| match p.as_rule() {
        Rule::DDL => Some(Sql::DDL(DDL::from_pair(p))),
        Rule::DML => Some(Sql::DML(DML::from_pair(p))),
        Rule::DQL => Some(Sql::DQL(DQL::from_pair(p))),
        Rule::comment => Some(Sql::Comment(Comment::from_pair(p))),
        _ => None,
    })
    .collect()
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes with tests
4. Run the test suite: `cargo test`
5. Commit your changes: `git commit -m 'Add amazing feature'`
6. Push to the branch: `git push origin feature/amazing-feature`
7. Open a Pull Request

### Commit Convention

- `feat:` new features
- `fix:` bug fixes
- `refactor:` code refactoring
- `test:` adding tests
- `docs:` documentation updates
- `chore:` maintenance tasks

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Pest](https://pest.rs/) parser generator
- Inspired by nature's organizational patterns
- Uses functional programming principles from the Rust ecosystem

---

**Version**: 0.1.1
**Rust Edition**: 2024
**Minimum Rust Version**: 1.70+
