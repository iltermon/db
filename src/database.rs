use mysql::{prelude::Queryable, Pool, PooledConn};

pub struct DatabaseStruct {
    // pub is_connected: bool,
    // pub password: String,
    // pub user: String,
    // pub port: String,
    // pub host: String,
    connection: PooledConn,
}

pub fn connect() -> DatabaseStruct {
    let url = "mysql://root:password@localhost:3306/sys";
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
        let selected_payments: Vec<String> = self.connection.query("select 'test'").unwrap();
        print!("{}", &selected_payments[0].to_string());
    }
}
