use mysql::{Pool, PooledConn};

pub fn get_db_connection() -> Option<PooledConn> {
    let url = "mysql://root:password@localhost:3306/sys";
    let pool = Pool::new(url).unwrap();

    let conn = pool.get_conn().ok();

    //let selected_payments:Vec<i32> = conn.query("select 1").unwrap();
    //print!("{}",&selected_payments[0].to_string());
    return conn;
}
