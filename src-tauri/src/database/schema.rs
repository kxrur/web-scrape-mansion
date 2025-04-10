// @generated automatically by Diesel CLI.

diesel::table! {
    mansionees (id) {
        id -> Nullable<Integer>,
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
        id -> Nullable<Integer>,
        mansionees_id -> Integer,
        name -> Text,
        path -> Text,
    }
}

diesel::joinable!(pictures -> mansionees (mansionees_id));

diesel::allow_tables_to_appear_in_same_query!(
    mansionees,
    pictures,
);
