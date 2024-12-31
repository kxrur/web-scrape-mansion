use super::models::*;
use super::schema::mansions;
use diesel::prelude::*;
use std::io::{stdin, Read};
use std::{env, panic};

use diesel::{Connection, PgConnection};
use dotenv::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_mansion(conn: &mut PgConnection, new_mansion: NewMansion) -> Mansion {
    diesel::insert_into(mansions::table)
        .values(&new_mansion)
        .returning(Mansion::as_returning())
        .get_result(conn)
        .expect("Error saving new mansion")
}

pub fn read_mansions() {
    use super::schema::mansions::dsl::*;

    let connection = &mut establish_connection();

    let results = mansions
        .limit(5)
        .select(Mansion::as_select())
        .load(connection)
        .expect("Error loading mansions");

    println!("Displaying {} mansions", results.len());
    for mansion in results {
        println!("{}", mansion.address);
        print!("-----------");
        println!("{}", mansion.id);
    }
}

pub fn test_postsql() {
    let connection = &mut establish_connection();

    let street = "some street";

    let mansion = create_mansion(connection, {
        NewMansion {
            address: street.to_string(),
            price: "987$".to_string(),
            size: "123 sq ft".to_string(),
            bedrooms: 1,
            bathrooms: 2,
            receptions: 3,
            type_: "castle".to_string(),
        }
    });
    println!("\nSaved mansion {street} with id {}", mansion.id);

    read_mansions()
}
#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
