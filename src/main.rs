use sqlparser::ast;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

fn main() {
    println!("Hello, world!");
    parseSql()
}

fn parseSql() {
    let sql = "SELECT a, b, 123, myfunc(b) \
           FROM table_1 \
           WHERE a > b AND b < 100 \
           ORDER BY a DESC, b";

    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

    let ast_res = Parser::parse_sql(&dialect, sql)?;
    for name in ast_res {
        match name {}
    }

    println!("AST: {:?}", ast);
}
