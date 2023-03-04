

use mysql::{Pool, prelude::{Queryable}};

pub fn connect(){
    let url = "mysql://root:password@localhost:3306/sys";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();
    let selected_payments:Vec<i32> = conn.query("select 1").unwrap();
    print!("{}",&selected_payments[0].to_string());
        
}