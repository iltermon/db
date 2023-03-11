use database::DatabaseTrait;
mod database;
mod util;
fn main() {
    let database_url=util::get_parameter("database_url");
    let mut database_obj = database::connect(&database_url);
     database_obj.select_from("dual", "'test'");
}
