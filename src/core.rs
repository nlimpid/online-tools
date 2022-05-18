use convert_case::{Case, Casing};
use sqlparser::ast::DataType;
use sqlparser::ast::Statement;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::{Parser, ParserError};
use std::io::BufRead;

pub struct SqlItem {
    table_name: String,
    columns: Vec<Column>,
}

pub struct Column {
    name: String,
    sql_type: String,
}

pub fn parse_sql(sql_str: String) -> SqlItem {
    let mut res = SqlItem {
        columns: Vec::new(),
        table_name: "".to_string(),
    };

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
        Statement::CreateTable {
            or_replace,
            temporary,
            external,
            if_not_exists,
            name,
            columns,
            ..
        } => {
            res.table_name = name.to_string();

            for c in columns.into_iter() {
                let data_type = &c.data_type;
                res.columns.push(Column {
                    name: c.name.to_string(),
                    sql_type: sql_type_to_go(data_type),
                });
                println!(
                    "column : {:?} type: {}, converted_type: {}\n",
                    c.name,
                    data_type,
                    sql_type_to_go(data_type)
                )
            }
        }
        _ => println!("Default case, x = {:?}", sss),
    }

    return res;

    // println!("AST: {:?}", ast);
}

fn sql_type_to_go(typ: &DataType) -> String {
    let mut res: String = "".to_string();
    match typ {
        DataType::String => res = "string".to_string(),
        DataType::Varchar(_) => res = "string".to_string(),
        DataType::Text => res = "string".to_string(),
        DataType::Int(_) => res = "int".to_string(),
        DataType::UnsignedInt(_) => res = "uint".to_string(),
        DataType::BigInt(_) => res = "int64".to_string(),
        DataType::UnsignedBigInt(_) => res = "uint64".to_string(),
        DataType::Timestamp => res = "time.Time".to_string(),
        DataType::Float(_) => res = "decimal.Decimal".to_string(),
        DataType::Double => res = "decimal.Decimal".to_string(),
        DataType::Custom(o) => {
            if o.to_string() == "bigserial" {
                res = "int64".to_string()
            }
        }
        _ => res = "string".to_string(),
    }
    return res;
}

fn gen_go_code(item: SqlItem) -> String {
    let tbl_name = item.table_name;
    let mut vec = Vec::new();
    vec.push(format!("type {} struct {{", tbl_name));
    for c in item.columns {
        vec.push(format!("{} {}", c.name.to_case(Case::Pascal), c.sql_type));
    }
    vec.push("}}".to_string());
    return vec.join("\n");
}

#[cfg(test)]
mod tests {
    use crate::core::{gen_go_code, parse_sql};

    #[test]
    fn test_parse_sql() {
        let sql = r#"CREATE TABLE ipo_hk
    (
        id                bigserial NOT NULL PRIMARY KEY,
        created_at          timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP 
    )"#
        .to_string();
        let got = gen_go_code(parse_sql(sql));
        println!("{}", got)
    }
}
