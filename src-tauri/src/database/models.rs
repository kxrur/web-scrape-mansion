// src/models.rs
#![allow(unused)]
#![allow(clippy::all)]

use super::schema::{mansionees, pictures, settings};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct Picture {
    pub path: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Queryable, Selectable, Debug, Insertable, Clone, Type)]
#[diesel(table_name = mansionees)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewMansionee {
    pub address: String,
    pub price: Option<i32>,
    pub size: Option<f32>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub receptions: Option<i32>,
    pub house_type: String,
    pub uuid: Vec<u8>,
}

impl NewMansionee {
    pub fn new(
        address: String,
        price: Option<i32>,
        size: Option<f32>,
        bedrooms: Option<i32>,
        bathrooms: Option<i32>,
        receptions: Option<i32>,
        house_type: String,
    ) -> Self {
        let uuid = Uuid::new_v4().as_bytes().to_vec();
        NewMansionee {
            address,
            price,
            size,
            bedrooms,
            bathrooms,
            receptions,
            house_type,
            uuid,
        }
    }

    pub fn log(&self) {
        let uuid = Uuid::from_slice(&self.uuid).unwrap_or(Uuid::nil());

        println!(
            "Here is a mansion: 
            Address: {}
            Price: {} 
            Size: {} sqm
            Bedrooms: {} 
            Bathrooms: {} 
            Receptions: {} 
            Type: {}
            UUID: {}",
            self.address,
            self.price.map_or("N/A".to_string(), |p| format!("${}", p)),
            self.size.map_or("N/A".to_string(), |s| format!("{:.2}", s)),
            self.bedrooms.map_or("N/A".to_string(), |b| b.to_string()),
            self.bathrooms.map_or("N/A".to_string(), |b| b.to_string()),
            self.receptions.map_or("N/A".to_string(), |r| r.to_string()),
            self.house_type,
            uuid
        );
    }
}

#[derive(Serialize, Deserialize, Queryable, Selectable, Debug, Clone, Type, Identifiable)]
#[diesel(table_name = mansionees)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Mansionee {
    pub id: i32,
    pub uuid: Vec<u8>,
    pub address: String,
    pub price: Option<i32>,
    pub size: Option<f32>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub receptions: Option<i32>,
    pub house_type: String,
}

impl Mansionee {
    pub fn log(&self) {
        let uuid = Uuid::from_slice(&self.uuid).unwrap_or(Uuid::nil());

        println!(
            "Here is a mansion: 
            ID: {}
            UUID: {}
            Address: {}
            Price: {} 
            Size: {} sqm
            Bedrooms: {} 
            Bathrooms: {} 
            Receptions: {} 
            Type: {}",
            self.id,
            uuid,
            self.address,
            self.price.map_or("N/A".to_string(), |p| format!("${}", p)),
            self.size.map_or("N/A".to_string(), |s| format!("{:.2}", s)),
            self.bedrooms.map_or("N/A".to_string(), |b| b.to_string()),
            self.bathrooms.map_or("N/A".to_string(), |b| b.to_string()),
            self.receptions.map_or("N/A".to_string(), |r| r.to_string()),
            self.house_type
        );
    }
}

#[derive(Serialize, Deserialize, Queryable, Selectable, Debug, Clone, Type, Insertable)]
#[diesel(table_name = pictures)]
pub struct NewPicture {
    pub mansionee_id: i32,
    pub name: String,
    pub path: String,
}

#[derive(
    Serialize, Deserialize, Queryable, Selectable, Debug, Clone, Type, Associations, Identifiable,
)]
#[diesel(table_name = pictures)]
#[diesel(belongs_to(Mansionee))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DbPicture {
    pub id: i32,
    pub mansionee_id: i32,
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Queryable, Selectable, Debug, Insertable, Clone, Type)]
#[diesel(table_name = settings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewSetting {
    pub profile: Option<String>,
    pub theme: Option<String>,
    pub db_path: Option<String>,
}

#[derive(
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    Debug,
    Insertable,
    Clone,
    Type,
    Default,
    AsChangeset,
)]
#[diesel(table_name = settings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Setting {
    pub id: i32,
    pub profile: Option<String>,
    pub theme: Option<String>,
    pub db_path: Option<String>,
}
