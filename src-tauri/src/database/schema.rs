// @generated automatically by Diesel CLI.

diesel::table! {
    mansionees (id) {
        id -> Integer,
        address -> Text,
        price -> Nullable<Integer>,
        size -> Nullable<Float>,
        bedrooms -> Nullable<Integer>,
        bathrooms -> Nullable<Integer>,
        receptions -> Nullable<Integer>,
        house_type -> Text,
        uuid -> Binary,
    }
}

diesel::table! {
    pictures (id) {
        id -> Integer,
        mansionee_id -> Integer,
        name -> Text,
        path -> Text,
    }
}

diesel::table! {
    settings (id) {
        id -> Integer,
        profile -> Nullable<Text>,
        theme -> Nullable<Text>,
        db_path -> Nullable<Text>,
    }
}

diesel::joinable!(pictures -> mansionees (mansionee_id));

diesel::allow_tables_to_appear_in_same_query!(
    mansionees,
    pictures,
    settings,
);
