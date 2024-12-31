// @generated automatically by Diesel CLI.

diesel::table! {
    images (id) {
        id -> Int4,
        image_path -> Text,
        mansion_id -> Int4,
    }
}

diesel::table! {
    mansions (id) {
        id -> Int4,
        address -> Text,
        price -> Text,
        size -> Text,
        bedrooms -> Int4,
        bathrooms -> Int4,
        receptions -> Int4,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

diesel::joinable!(images -> mansions (mansion_id));

diesel::allow_tables_to_appear_in_same_query!(
    images,
    mansions,
);
