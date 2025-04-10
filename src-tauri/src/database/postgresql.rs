use diesel::{Connection, SqliteConnection};
use dotenv::dotenv;
use std::{env, panic, sync::Mutex};

use crate::AppState;

use super::{
    models::{Mansionee, NewMansionee, NewSetting, Setting},
    schema::{
        mansionees,
        settings::{self, id},
    },
};
use diesel::prelude::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
pub fn save_mansionee_to_database(new_mansion: NewMansionee) -> Mansionee {
    let conn = &mut establish_connection();

    diesel::insert_into(mansionees::table)
        .values(&new_mansion)
        .returning(Mansionee::as_returning())
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
pub fn save_setting(new_setting: NewSetting) -> Option<Setting> {
    let connection = &mut establish_connection();
    match diesel::insert_into(settings::table)
        .values(&new_setting)
        .returning(Setting::as_returning())
        .get_result(connection)
    {
        Ok(setting) => Some(setting),
        Err(e) => {
            println!("Error saving the setting: {}", e);
            None
        }
    }
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
            if !all_settings.is_empty() {
                let mut state = state.lock().unwrap();
                state.settings = all_settings.clone();
            }
            Some(all_settings)
        }
        Err(e) => {
            println!("Error loading the settings: {}", e);
            None
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn update_setting(setting: Setting) -> Option<u32> {
    let connection = &mut establish_connection();

    match diesel::update(settings::table)
        .filter(id.eq(setting.id))
        .set(&setting)
        .execute(connection)
    {
        Ok(num_rows) => Some(num_rows as u32),
        Err(e) => {
            println!("Error updating the setting: {}", e);
            None
        }
    }
}
