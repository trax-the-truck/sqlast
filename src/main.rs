use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use std::env;
use std::fmt;
use std::process;

#[derive(Debug)]
pub struct SqlAstError {
    reason: String,
}

impl fmt::Display for SqlAstError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SqlAstError: {}", self.reason)
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let result = if args.len() == 3 {
        match args[1].as_ref() {
            "parse" => parse(&args[2]),
            "compose" => compose(&args[2]),
            other => Err(SqlAstError {
                reason: format!("Unknown command {}", other),
            }),
        }
    } else {
        Err(SqlAstError {
            reason: "Invalid number of args.".to_string(),
        })
    };

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
    let dialect = PostgreSqlDialect {};

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

    match serde_json::to_string(&asts[0]) {
        Ok(json) => Ok(json),
        Err(serialization_error) => Err(SqlAstError {
            reason: format!("Serialization error: {}", serialization_error),
        }),
    }
}

/// Compose a SQL query from a json AST.
pub fn compose(obj_json: &str) -> Result<String, SqlAstError> {
    match serde_json::from_str::<sqlparser::ast::Statement>(obj_json) {
        Ok(stmt) => Ok(format!("{}", stmt)),
        Err(deserialization_error) => Err(SqlAstError {
            reason: format!("Error deserializing AST: {}", deserialization_error),
        }),
    }
}
