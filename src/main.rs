pub mod models;
mod database;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;


fn main() {
    let connection = database::establish_connection();
}
