use convert_case::{Case, Casing};
use sqlparser::ast::DataType;
use sqlparser::ast::Statement;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::{Parser, ParserError};
use std::error::Error;

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
        Statement::CreateTable { name, columns, .. } => {
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
        DataType::TinyInt(_) => res = "int32".to_string(),
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
        DataType::Decimal(_, _) => res = "decimal.Decimal".to_string(),
        DataType::Custom(o) => {
            print!("object name is {}", o.to_string());
            match o.to_string().as_str() {
                "jsonb" => res = "JSONB".to_string(),
                "bigserial" => res = "int64".to_string(),
                _ => res = "string".to_string(),
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
        vec.push(format!(
            "\t{}\t{}",
            c.name.to_case(Case::Pascal),
            c.sql_type
        ));
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
        let sql = r#"CREATE TABLE "public"."ipo_us_audit_list"
        (
            "id"                          bigserial    NOT NULL PRIMARY KEY,
            "created_at"                  timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
            "updated_at"                  timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
            "cik"                         varchar(128) NOT NULL DEFAULT '',
            "temp_code"                   varchar(255) NOT NULL,
            "formal_code"                 varchar(255) NOT NULL,
            "form_stage"                  varchar(128) NOT NULL,
            "preliminary_prospectus_time" timestamp NOT NULL,
            "amend_prospectus_time"       timestamp NOT NULL
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

    #[test]
    fn test_parse_pg_type() {
        let sql = r#"create table public.data_manager_task_history
        (
            id           bigserial,
            created_at   timestamp default CURRENT_TIMESTAMP not null,
            updated_at   timestamp default CURRENT_TIMESTAMP not null,
            task_name varchar(255),
            task_time timestamp,
            status varchar(255),
            statistic_content jsonb
        )
        "#
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

    #[test]
    fn test_parse_pg_type2() {
        let sql = r#"CREATE TABLE "public"."ipos"
        (
            "id"                     bigserial    NOT NULL PRIMARY KEY,
            "created_at"             timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
            "updated_at"             timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
            "counter_id"             varchar(255) NOT NULL,
            "code"                   varchar(255) NOT NULL,
            "market"                 varchar(255) NOT NULL,
            "ipo_date"               bigint       NOT NULL,
            "issue_price"            numeric,
            "issue_price_min"        numeric,
            "issue_price_max"        numeric,
            "issue_currency"         varchar(255) NOT NULL DEFAULT 0,
            "prospectus"             varchar(255) NOT NULL DEFAULT 0,
            "total_shares"           numeric      NOT NULL DEFAULT 0,
            "sort"                   int8         NOT NULL DEFAULT 0,
            "state"                  int4         NOT NULL DEFAULT 0,
            "apply_start_date"       bigint       NOT NULL DEFAULT 0,
            "pay_end_date"           bigint       NOT NULL DEFAULT 0,
            "issue_result_publ_date" bigint       NOT NULL DEFAULT 0,
            "pspl_mart_begin"        bigint       NOT NULL DEFAULT 0,
            "pspl_mart_end"          bigint       NOT NULL DEFAULT 0,
            "show_mart"              int4         NOT NULL DEFAULT 0,
            "rec_purposes"           jsonb
        );
        "#
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
