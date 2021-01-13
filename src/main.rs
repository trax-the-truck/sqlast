use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::env;
use std::fmt;
use std::io::{self, Read};
use std::process;

// A custom error type
// Used to represent any usage/serialization/parsing errors
// We could just use a string instead, but this should be more explicit.
pub struct SqlAstError {
    reason: String,
}

// This is the equivalent of a to string method.
// It's what format! and println! use
impl fmt::Display for SqlAstError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SqlAstError: {}", self.reason)
    }
}

fn run(mut args: Vec<String>) -> Result<String, SqlAstError> {
    if args.len() == 2 && args[1] == "--help" {
        return Ok(
            format!("sqlast {}\n", env!("CARGO_PKG_VERSION")) +
            "Usage: sqlast <parse|compose> <data>"
        )
    }

    // Usage `sqlast <parse|compose> <data>`
    if args.len() != 3 {
        return Err(SqlAstError {
            reason: "Invalid number of args. Try --help".to_string(),
        });
    }

    // Rewrite to support stdin
    if args[2] == "-" {
        args[2].clear();
        if io::stdin().read_to_string(&mut args[2]).is_err() {
            return Err(SqlAstError {
                reason: "Error reading from stdin.".to_string(),
            });
        }
    }

    match args[1].as_ref() {
        "parse" => parse(&args[2]),
        "compose" => compose(&args[2]),
        other => Err(SqlAstError {
            reason: format!("Unknown command {}", other),
        }),
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let result = run(args);

    match result {
        Ok(output) => println!("{}", output),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1)
        }
    }
}

/// Parse a sql query.
///
/// Returns either an error or an AST as json.
pub fn parse(sql: &str) -> Result<String, SqlAstError> {
    let dialect = GenericDialect {};

    // This will parse multiple queries and return a vector
    let asts = match Parser::parse_sql(&dialect, sql) {
        Ok(ast) => ast,
        Err(parse_error) => {
            return Err(SqlAstError {
                reason: format!("Parse error: {}", parse_error),
            });
        }
    };

    if asts.len() != 1 {
        return Err(SqlAstError {
            reason: "Multiple queries provided.".to_string(),
        });
    }

    // In case you haven't used Rust, note that there is no semicolon.
    // This is all one expression, and since it's the last thing it's the return value.
    match serde_json::to_string(&asts[0]) {
        Ok(json) => Ok(json),
        Err(serialization_error) => Err(SqlAstError {
            reason: format!("Serialization error: {}", serialization_error),
        }),
    }
}

/// Compose a SQL query from a json AST.
pub fn compose(obj_json: &str) -> Result<String, SqlAstError> {
    // serde is a general puropse serializing/deserializing library.
    // An optional feature of sqlparser is serde implementation on the AST.
    // This is a compile feature flag in Cargo.toml

    match serde_json::from_str::<sqlparser::ast::Statement>(obj_json) {
        Ok(stmt) => Ok(format!("{}", stmt)),
        Err(deserialization_error) => Err(SqlAstError {
            reason: format!("Error deserializing AST: {}", deserialization_error),
        }),
    }
}
