use super::schema::mansions;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = mansions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Mansion {
    pub id: i32,
    pub street: String,
    pub sold: bool,
}

#[derive(Insertable)]
#[diesel(table_name = mansions)]
pub struct NewMansion<'a> {
    pub street: &'a str,
    pub sold: &'a bool,
}
