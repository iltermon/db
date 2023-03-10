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
    fn run_select(&mut self);
}
impl DatabaseTrait for DatabaseStruct {
    fn run_select(&mut self) {
        let test_variable: Vec<String> = self.connection.query("select 'test'").unwrap();
        print!("{}", &test_variable[0].to_string());
    }
}
