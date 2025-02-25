// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use super::schema::mansionees;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;
#[derive(Serialize, Deserialize, Queryable, Selectable, Debug, Insertable, Clone, Type)]
#[diesel(table_name = mansionees)]
pub struct NewMansionee {
    pub address: String,
    pub price: Option<i32>,
    pub size: Option<f64>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub receptions: Option<i32>,
    pub house_type: String,
    pub pictures: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, Queryable)]
pub struct Picture {
    pub path: String,
    pub name: String,
}

impl NewMansionee {
    pub fn new(
        address: String,
        price: Option<i32>,
        size: Option<f64>,
        bedrooms: Option<i32>,
        bathrooms: Option<i32>,
        receptions: Option<i32>,
        house_type: String,
        pictures: Vec<Picture>,
    ) -> Self {
        let pictures_json = serde_json::to_value(pictures).ok();
        NewMansionee {
            address,
            price,
            size,
            bedrooms,
            bathrooms,
            receptions,
            house_type,
            pictures: pictures_json,
        }
    }
    pub fn log(&self) {
        println!(
            "Here is a mansion: 
            Address: {}
            Price: {} 
            Size: {} sqm
            Bedrooms: {} 
            Bathrooms: {} 
            Receptions: {} 
            Type: {}
            Pictures:{:?}",
            self.address,
            self.price.map_or("N/A".to_string(), |p| format!("${}", p)),
            self.size.map_or("N/A".to_string(), |s| format!("{:.2}", s)),
            self.bedrooms.map_or("N/A".to_string(), |b| b.to_string()),
            self.bathrooms.map_or("N/A".to_string(), |b| b.to_string()),
            self.receptions.map_or("N/A".to_string(), |r| r.to_string()),
            self.house_type,
            self.pictures
        );
    }
}

#[derive(Serialize, Deserialize, Queryable, Selectable, Debug, Clone, Type)]
#[diesel(table_name = mansionees)]
pub struct Mansionee {
    pub id: i32,
    pub uuid: Uuid,
    pub address: String,
    pub price: Option<i32>,
    pub size: Option<f64>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub receptions: Option<i32>,
    pub house_type: String,
    pub pictures: Option<serde_json::Value>,
}

impl Mansionee {
    pub fn log(&self) {
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
            Type: {}
            Pictures:{:?}",
            self.id,
            self.uuid,
            self.address,
            self.price.map_or("N/A".to_string(), |p| format!("${}", p)),
            self.size.map_or("N/A".to_string(), |s| format!("{:.2}", s)),
            self.bedrooms.map_or("N/A".to_string(), |b| b.to_string()),
            self.bathrooms.map_or("N/A".to_string(), |b| b.to_string()),
            self.receptions.map_or("N/A".to_string(), |r| r.to_string()),
            self.house_type,
            self.pictures
        );
    }
}
