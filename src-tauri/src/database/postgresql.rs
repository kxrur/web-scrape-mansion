use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use std::{env, panic, sync::Mutex};

use crate::AppState;

use super::{
    models::{Mansionee, NewMansionee, NewSetting, Setting},
    schema::{mansionees, settings},
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

#[tauri::command]
#[specta::specta]
pub fn save_setting(
    state: tauri::State<'_, Mutex<AppState>>,
    new_setting: NewSetting,
) -> Option<Vec<Setting>> {
    //FIXME: return None if the operation failed
    let connection = &mut establish_connection();
    diesel::insert_into(settings::table)
        .values(&new_setting)
        .returning(NewSetting::as_returning())
        .get_result(connection)
        .expect("Error saving new mansion");
    get_settings(state)
}

#[tauri::command]
#[specta::specta]
pub fn get_settings(state: tauri::State<'_, Mutex<AppState>>) -> Option<Vec<Setting>> {
    let connection = &mut establish_connection();

    match settings::dsl::settings
        .select(Setting::as_select())
        .load::<Setting>(connection)
    {
        Ok(all_settings) => {
            if all_settings.len() > 1 {
                let mut state = state.lock().unwrap();
                state.settings = all_settings.clone();
            }
            Some(all_settings)
        }
        Err(e) => {
            println!("Error loading the mansions: {}", e);
            None
        }
    }
}
