use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use std::{env, panic};

use super::{
    models::{Mansionee, NewMansionee},
    schema::mansionees,
};
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
pub fn save_mansionee(new_mansion: NewMansionee) -> NewMansionee {
    let conn = &mut establish_connection();

    diesel::insert_into(mansionees::table)
        .values(&new_mansion)
        .returning(NewMansionee::as_returning())
        .get_result(conn)
        .expect("Error saving new mansion")
}

pub fn get_mansionees() -> Option<Vec<Mansionee>> {
    use mansionees::dsl::mansionees;
    let connection = &mut establish_connection();

    match mansionees
        .select(Mansionee::as_select())
        .load::<Mansionee>(connection)
    {
        Ok(mansions) => Some(mansions),
        Err(e) => {
            println!("Error loading the mansions: {}", e);
            None
        }
    }
}
