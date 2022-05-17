use sqlparser::ast;
use sqlparser::ast::Statement;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::{Parser, ParserError};
use std::io::BufRead;

pub fn get_result() -> Result<(), ()> {
    return Ok(());
}

pub fn parse_sql(sql_str: String) {
    // let sql = "SELECT a, b, 123, myfunc(b) \
    //        FROM table_1 \
    //        WHERE a > b AND b < 100 \
    //        ORDER BY a DESC, b";

    let dialect = PostgreSqlDialect {};
    let ast: Result<Vec<Statement>, ParserError> = Parser::parse_sql(&dialect, sql_str.as_str());
    let ast = match ast {
        Ok(ast) => ast,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let sss = ast.iter().next().unwrap();
    // println!("Got: {}", val);
    match sss {
        // sqlparser::ast::Statement::CreateTable() => println!(ch  ()),
        Statement::CreateTable { name, columns, .. } => {
            println!("data: {:?}, clumns: {:?}", name, columns);

            for c in columns.into_iter() {
                println!("column : {:?}", c.name, c)
            }
        }
        _ => println!("Default case, x = {:?}", sss),
    }

    // println!("AST: {:?}", ast);
}

fn sql_type_to_go() {}

#[cfg(test)]
mod tests {
    use crate::core::parse_sql;

    #[test]
    fn test_parse_sql() {
        let sql = "CREATE TABLE ipo_hk \
    ( \
        id                bigserial NOT NULL PRIMARY KEY, \
        created_at          timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP \
    )"
        .to_string();
        parse_sql(sql);
    }
}
