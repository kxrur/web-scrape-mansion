// @generated automatically by Diesel CLI.

diesel::table! {
    mansions (id) {
        id -> Int4,
        street -> Varchar,
        sold -> Bool,
    }
}
