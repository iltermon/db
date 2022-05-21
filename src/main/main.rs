mod structs;
mod traits;
use regex::Regex;
use structs::*;
use traits::TableTrait;

impl Column {
    fn new_from(column: &Column) -> Column {
        Column {
            name: column.name.clone(),
            data_type: column.data_type.clone(),
            comment: column.comment.clone(),
            default: column.default.clone(),
        }
    }
    pub fn to_sql(&self) -> String {
        let mut sql = String::new();
        sql.push_str(&self.name);
        sql.push_str(" ");
        sql.push_str(&self.data_type);
        if !self.default.is_empty() {
            sql.push_str(" DEFAULT '");
            sql.push_str(&self.default);
            sql.push_str("'");
        }
        // if self.comment.len() > 0 {
        //     sql.push_str(" COMMENT ");
        //     sql.push_str(&self.comment);
        // }
        sql.push_str(",\n");
        return sql;
    }
}

impl TableTrait for Table {
    fn to_sql(&self) -> String {
        let mut sql = String::new();
        sql.push_str(&format!("CREATE TABLE {} (\n", self.name));
        for column in &self.columns {
            sql.push_str("\t");
            sql.push_str(&column.to_sql());
        }
        if !self
            .primary_key
            .unique_constraint
            .constraint
            .columns
            .is_empty()
        {
            sql.push_str(&self.primary_key.to_sql());
        }

        sql.pop();
        sql.push_str("\n)");
        return sql;
    }
}
impl Table {
    fn new(
        name: String,
        columns: Vec<Column>,
        constraints: Vec<structs::Constraint>,
        comment: String,
        primary_key: PrimaryKey,
    ) -> Table {
        return Table {
            name,
            columns,
            primary_key,
            constraints,
            comment,
        };
    }
}

impl PrimaryKey {
    fn to_sql(&self) -> String {
        let mut sql = String::new();
        let mut column_list = String::new();
        for column in &self.unique_constraint.constraint.columns {
            column_list.push_str(&column.name);
            column_list.push_str(",");
        }
        column_list.pop();
        sql.push_str(&format!(
            "\tCONSTRAINT {} PRIMARY KEY {}\n",
            self.unique_constraint.constraint.name, column_list,
        ));
        println!("{}", sql);
        return sql;
    }
}
fn get_table_name_from_sql(sql: &str) -> String {
    let re = Regex::new(r"CREATE *TABLE").unwrap();
    let mut table_name;
    if re.is_match(sql) {
        let match_start = re.find(sql).unwrap().start();
        let match_end = re.find(sql).unwrap().end();
        table_name = sql[match_end..sql.find('(').unwrap()].trim().to_string();
        return table_name;
    } else {
        println!("{}", "not match");
        return "".to_string();
    }
}
fn is_sql_valid(sql: &str) -> bool {
    let re = Regex::new(r"CREATE *TABLE").unwrap();
    if re.is_match(sql) {
        return true;
    } else {
        return false;
    }
}
fn parse_sql(sql: &str) {
    //for now only support sql server to oracle
    // table name : from the first word after CREATE TABLE to the first (
    if is_sql_valid(sql) == true {
        let table_name = get_table_name_from_sql(sql);
    } else {
        println!("{}", "sql is not valid");
    }
}
fn main() {
    let re = Regex::new(r"CREATE *TABLE").unwrap();
    println!("{}", re.find("CREATE TABLE").unwrap().start().to_string());
    println!("{}", re.is_match("CREATE TABLE"));
    println!("{}", "CREATE TABLE       xxxxxxxxxx (".find('(').unwrap());
}
