use diesel::{Connection, SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::{env, fs, panic, sync::Mutex};
use tauri::Manager;

use crate::AppState;

use super::{
    models::{DbPicture, Mansionee, NewMansionee, NewPicture, NewSetting, Setting},
    schema::{
        mansionees, pictures,
        settings::{self, id},
    },
};
use diesel::prelude::*;

#[tauri::command]
pub fn init_database(app_handle: &tauri::AppHandle) {
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    let app_dir = app_handle
        .path()
        .app_data_dir()
        .expect("failed to retrieve data dir");

    fs::create_dir_all(&app_dir).expect("failed to create data dir");

    let db_path = app_dir.join("mansions.db");
    env::set_var("DATABASE_URL", db_path);

    let mut connection = establish_connection();
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error migrating");
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
pub fn save_mansionee(new_mansion: NewMansionee) -> Option<Mansionee> {
    let conn = &mut establish_connection();

    match diesel::insert_into(mansionees::table)
        .values(&new_mansion)
        .returning(Mansionee::as_returning())
        .get_result(conn)
    {
        Ok(mansion) => Some(mansion),
        Err(e) => {
            println!("error saving the mansion: {}", e);
            None
        }
    }
}

pub fn save_pictures(new_pictures: Vec<NewPicture>) -> Option<Vec<DbPicture>> {
    let conn = &mut establish_connection();
    let mut pictures: Vec<DbPicture> = Vec::new();

    for new_picture in new_pictures {
        println!("pushing picture?");
        match diesel::insert_into(pictures::table)
            .values(&new_picture)
            .returning(DbPicture::as_returning())
            .get_result(conn)
        {
            Ok(picture) => pictures.push(picture),
            Err(e) => {
                println!("error saving pictures: {}", e);
                return None;
            }
        };
    }
    Some(pictures)
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

pub fn get_mansionee(mansion_id: i32) -> Option<Mansionee> {
    use mansionees::dsl::mansionees;
    let connection = &mut establish_connection();

    mansionees
        .find(mansion_id)
        .select(Mansionee::as_select())
        .first::<Mansionee>(connection)
        .optional()
        .unwrap_or_else(|e| {
            println!("Error loading mansion {}: {}", mansion_id, e);
            None
        })
}

pub fn get_pictures(mansionee: &Mansionee) -> Option<Vec<DbPicture>> {
    let connection = &mut establish_connection();

    match DbPicture::belonging_to(mansionee)
        .select(DbPicture::as_select())
        .load(connection)
    {
        Ok(pics) => Some(pics),
        Err(e) => {
            println!("Error loading pictures for mansion {}: {}", mansionee.id, e);
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
pub fn save_setting(
    new_setting: NewSetting,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Option<Setting> {
    let connection = &mut establish_connection();
    match diesel::insert_into(settings::table)
        .values(&new_setting)
        .returning(Setting::as_returning())
        .get_result(connection)
    {
        Ok(setting) => {
            let mut state = state.lock().unwrap();
            state.settings.insert(0, setting.clone());
            Some(setting)
        }
        Err(e) => {
            println!("Error saving the setting: {}", e);
            None
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn update_setting(setting: Setting, state: tauri::State<'_, Mutex<AppState>>) -> Option<u32> {
    let connection = &mut establish_connection();

    match diesel::update(settings::table)
        .filter(id.eq(setting.id))
        .set(&setting)
        .execute(connection)
    {
        Ok(num_rows) => {
            let mut state = state.lock().unwrap();
            state.settings.insert(0, setting.clone());
            Some(num_rows as u32)
        }
        Err(e) => {
            println!("Error updating the setting: {}", e);
            None
        }
    }
}
