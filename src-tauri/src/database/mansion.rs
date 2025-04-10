use super::{
    models::{DbPicture, Mansionee, NewMansionee, NewPicture},
    sqlite::{save_mansionee_to_database, save_pictures_to_database},
};

struct TheNewMansion {
    pub address: String,
    pub price: Option<i32>,
    pub size: Option<f32>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub receptions: Option<i32>,
    pub house_type: String,
    pub uuid: Vec<u8>,
    pub pictures: Vec<NewPicture>,
}

impl TheNewMansion {
    pub fn to_new_mansionee(&self) -> NewMansionee {
        NewMansionee::new(
            self.address.clone(),
            self.price,
            self.size,
            self.bedrooms,
            self.bathrooms,
            self.receptions,
            self.house_type.clone(),
            self.uuid.clone(),
        )
    }

    pub fn to_new_pictures(&self, mansion_id: i32) -> Vec<NewPicture> {
        let mut new_pictures: Vec<NewPicture> = Vec::new();
        self.pictures.iter().map(|picture| {
            let picture = picture.clone();
            let new_picture = NewPicture {
                path: picture.path,
                name: picture.name,
                mansionees_id: mansion_id,
            };
            new_pictures.push(new_picture);
        });
        new_pictures
    }
}

struct TheMansion {
    pub id: i32,
    pub address: String,
    pub price: Option<i32>,
    pub size: Option<f32>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub receptions: Option<i32>,
    pub house_type: String,
    pub uuid: Vec<u8>,
    pub pictures: Vec<DbPicture>,
}

impl TheMansion {
    pub fn construct_mansion(mansionee: Mansionee, pictures: Vec<DbPicture>) -> TheMansion {
        TheMansion {
            id: mansionee.id,
            pictures,
            uuid: mansionee.uuid,
            house_type: mansionee.house_type,
            receptions: mansionee.receptions,
            bathrooms: mansionee.bathrooms,
            bedrooms: mansionee.bedrooms,
            size: mansionee.size,
            price: mansionee.price,
            address: mansionee.address,
        }
    }
}

pub fn save_mansion(mansion: TheNewMansion) -> TheMansion {
    let new_mansionee = mansion.to_new_mansionee();
    let mansionee = save_mansionee_to_database(new_mansionee);
    let new_pictures = mansion.to_new_pictures(mansionee.id);
    let pictures = save_pictures_to_database(new_pictures);
    TheMansion::construct_mansion(mansionee, pictures)
}
