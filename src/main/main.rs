mod structs;
mod traits;
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
            sql.push_str(" DEFAULT ");
            sql.push_str(&self.default);
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
        if !self.primary_key.unique_constraint.constraint.columns.is_empty() {
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

fn main() {
    let mut columns: Vec<Column> = Vec::new();
    let mut pk_columns: Vec<Column> = Vec::new();
    let column_id = Column {
        name: "id".to_string(),
        data_type: "integer".to_string(),
        comment: "".to_string(),
        default: "".to_string(),
    };
    let column_name = Column {
        name: "name".to_string(),
        data_type: "varchar(255)".to_string(),
        comment: "".to_string(),
        default: "".to_string(),
    };
    columns.push(Column::new_from(&column_id));
    columns.push(column_name);
    pk_columns.push(Column::new_from(&column_id));
    let primary_key = PrimaryKey {
        unique_constraint: UniqueConstraint {
            constraint: Constraint {
                name: "pk".to_string(),
                columns: pk_columns,
            },
        },
    };
    let table = Table::new(
        "TEST".to_string(),
        columns,
        Vec::new(),
        "".to_string(),
        primary_key,
    );
    print!("{}", table.to_sql());
}
