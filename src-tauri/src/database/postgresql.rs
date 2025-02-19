use std::{env, panic};

use diesel::{Connection, PgConnection, QueryResult};
use dotenv::dotenv;

use super::{models::Mansionee, schema::mansionees};
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
pub fn save_mansionee(new_mansion: Mansionee) -> Mansionee {
    let conn = &mut establish_connection();

    diesel::insert_into(mansionees::table)
        .values(&new_mansion)
        .returning(Mansionee::as_returning())
        .get_result(conn)
        .expect("Error saving new mansion")
}
