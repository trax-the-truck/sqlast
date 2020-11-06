use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use std::env;
// use std::fmt;

// #[derive(Debug)]
// struct SqlAstError {
//     reason: String
// }

// impl fmt::Display for SqlAstError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "SqlAstError: {}", self.reason)
//     }
// }

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        panic!("Invalid number of args!");
    }

    match args[1].as_ref() {
        "parse" => println!("{}", parse(&args[2]).unwrap()),
        "compose" => println!("{}", compose(&args[2]).unwrap()),
        other => panic!("Unknown command {}", other)
    };
}

pub fn parse(sql: &str) -> Result<String, sqlparser::parser::ParserError> {
    let dialect = PostgreSqlDialect {};

    let ast = Parser::parse_sql(&dialect, sql)?;

    let serialized = serde_json::to_string(&ast[0]).unwrap();

    Ok(serialized)
}

pub fn compose(obj_json: &str) -> Result<String, serde_json::Error> {
    let stmt = serde_json::from_str::<sqlparser::ast::Statement>(obj_json)?;

    Ok(format!("{}", stmt))
}
