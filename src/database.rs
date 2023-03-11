use mysql::{prelude::Queryable, Pool, PooledConn};

pub struct DatabaseStruct {
    // pub is_connected: bool,
    // pub password: String,
    // pub user: String,
    // pub port: String,
    // pub host: String,
    connection: PooledConn,
}

pub fn connect(url: &str) -> DatabaseStruct {
    let pool = Pool::new(url).unwrap();
    let return_object = DatabaseStruct {
        connection: pool.get_conn().unwrap(),
    };

    return return_object;
}

pub trait DatabaseTrait {
    fn select_from(&mut self, table_name: &str, column_list: &str);
}
impl DatabaseTrait for DatabaseStruct {
    fn select_from(&mut self, table_name: &str, column_list: &str) {
        let query_base = "select ";
        let query = format!("{} {} from {}", query_base, column_list, table_name);
        let test_variable: Vec<String> = self.connection.query(query).unwrap();
        print!("{}", &test_variable[0].to_string());
    }
}
