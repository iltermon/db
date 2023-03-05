use database::DatabaseTrait;

mod database;
fn main() {
    let mut database_obj = database::connect();
    database_obj.run_select();
}
