use convert_case::{Case, Casing};
use sqlparser::ast::DataType;
use sqlparser::ast::Statement;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::{Parser, ParserError};
use std::error::Error;
use std::io::BufRead;

pub struct SqlItem {
    table_name: String,
    columns: Vec<Column>,
}

pub struct Column {
    name: String,
    sql_type: String,
}

pub fn parse_sql(sql_str: String) -> Result<SqlItem, Box<dyn Error>> {
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
            if let Some(table_name) = name.0.last() {
                res.table_name = table_name.clone().value
            }

            for c in columns.into_iter() {
                let data_type = &c.data_type;
                res.columns.push(Column {
                    name: c.name.value.to_ascii_lowercase(),
                    // name: c.name.to_string(),
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

    return Ok(res);

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

pub fn gen_go_code(item: SqlItem) -> String {
    let tbl_name = item.table_name;
    let mut vec = Vec::new();
    vec.push(format!("type {} struct {{", tbl_name.to_case(Case::Pascal)));
    for c in item.columns {
        vec.push(format!("\t{}\t{}", c.name.to_case(Case::Pascal), c.sql_type));
    }
    vec.push("}}".to_string());
    return vec.join("\n");
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{gen_go_code, parse_sql},
        gen,
    };

    #[test]
    fn test_parse_sql() {
        let sql = r#"CREATE TABLE "public"."ice_ca_listing"
(
    "id"                         bigserial NOT NULL PRIMARY KEY,
    "created_at"                 timestamp          DEFAULT CURRENT_TIMESTAMP,
    "updated_at"                 timestamp          DEFAULT CURRENT_TIMESTAMP,
    "entry_date"                 varchar(255),
    "event_id"                   varchar(255),
    "revision"                   varchar(255),
    "id_region"                  varchar(255),
    "cusip"                      varchar(255),
    "isin"                       varchar(255),
    "ticker"                     varchar(255),
    "security_type"              varchar(255),
    "instrument_id"              varchar(255),
    "instrument"                 jsonb     NOT NULL DEFAULT '{}'::jsonb,
    "event_type"                 varchar(255),
    "action_type"                varchar(255),
    "record_date"                varchar(255),
    "event_status"               varchar(255),
    "event_description"          text,
    "announcement_date"          varchar(255),
    "announcement_status"        varchar(255),
    "declaration_date"           varchar(255),
    "expiration_date"            varchar(255),
    "effective_date"             varchar(255),
    "ex_date"                    varchar(255),
    "payment_date"               varchar(255),
    "dealing_date"               varchar(255),
    "event_market_details"       jsonb     NOT NULL DEFAULT '{}'::jsonb,
    "listing_type"               varchar(255),
    "listing_market_code"        varchar(255),
    "listing_market_description" varchar(255),
    "new_market_code"            varchar(255),
    "new_market_description"     varchar(255)
)"#
        .to_string();
        let inner_result = parse_sql(sql);
        match inner_result {
            Ok(res) => {
                let got = gen_go_code(res);
                println!("{}", got)
            }
            Err(e) => println!("err {}", e),
        }
    }
}
