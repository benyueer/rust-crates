use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn conn() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("no DATABASE_URL");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
