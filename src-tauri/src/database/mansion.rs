use super::{
    models::{DbPicture, Mansionee, NewMansionee, NewPicture, Picture},
    sqlite::{get_mansionee, get_mansionees, get_pictures, save_mansionee, save_pictures},
};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct MansionWithPictures<M, P> {
    pub mansion: M,
    pub pictures: Vec<P>,
}

pub type NewMansion = MansionWithPictures<NewMansionee, Picture>;
pub type Mansion = MansionWithPictures<Mansionee, DbPicture>;

impl Mansionee {
    pub fn with_pictures(self, pictures: Vec<DbPicture>) -> Mansion {
        MansionWithPictures {
            mansion: self,
            pictures,
        }
    }
}

impl NewMansionee {
    pub fn with_pictures(self, pictures: Vec<Picture>) -> NewMansion {
        MansionWithPictures {
            mansion: self,
            pictures,
        }
    }
}

pub fn get_mansions() -> Option<Vec<Mansion>> {
    let mut mansions = Vec::new();
    if let Some(mansionees) = get_mansionees() {
        for mansionee in mansionees {
            if let Some(pictures) = get_pictures(&mansionee) {
                mansions.push(mansionee.with_pictures(pictures))
            }
        }
    }
    Some(mansions)
}

pub fn get_mansion(id: i32) -> Option<Mansion> {
    if let Some(mansionee) = get_mansionee(id) {
        if let Some(pictures) = get_pictures(&mansionee) {
            return Some(mansionee.with_pictures(pictures));
        }
    }
    None
}

pub fn save_mansion(new_mansion: NewMansion) -> Option<Mansion> {
    let mansionee = save_mansionee(new_mansion.mansion)?;

    let mut pictures: Vec<NewPicture> = Vec::new();
    for picture in new_mansion.pictures.into_iter() {
        pictures.push(NewPicture {
            path: picture.path,
            mansionee_id: mansionee.id,
            name: picture.name,
        })
    }

    println!("start picture saving");
    let db_pictures = save_pictures(pictures)?;

    Some(MansionWithPictures {
        mansion: mansionee,
        pictures: db_pictures,
    })
}
