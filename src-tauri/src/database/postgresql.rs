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

pub fn create_mansion(conn: &mut PgConnection, street: &str, sold: &bool) -> Mansion {
    let new_post = NewMansion { street, sold };

    diesel::insert_into(mansions::table)
        .values(&new_post)
        .returning(Mansion::as_returning())
        .get_result(conn)
        .expect("Error saving new mansion")
}

pub fn read_mansions() {
    use super::schema::mansions::dsl::*;

    let connection = &mut establish_connection();

    let results = mansions
        .filter(sold.eq(true))
        .limit(5)
        .select(Mansion::as_select())
        .load(connection)
        .expect("Error loading mansions");

    println!("Displaying {} mansions", results.len());
    for mansion in results {
        println!("{}", mansion.street);
        print!("-----------");
        println!("{}", mansion.id);
    }
}

pub fn test_postsql() {
    let connection = &mut establish_connection();

    let mut street = String::new();
    let sold = true;

    println!("What would you like your street to be?");
    stdin().read_line(&mut street).unwrap();
    let street = street.trim_end(); // Remove the trailing newline

    let post = create_mansion(connection, street, &sold);
    println!("\nSaved mansion {street} with id {}", post.id);

    read_mansions()
}
#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
