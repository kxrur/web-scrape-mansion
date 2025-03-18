// @generated automatically by Diesel CLI.

diesel::table! {
    mansionees (id) {
        id -> Int4,
        address -> Text,
        price -> Nullable<Int4>,
        size -> Nullable<Float8>,
        bedrooms -> Nullable<Int4>,
        bathrooms -> Nullable<Int4>,
        receptions -> Nullable<Int4>,
        house_type -> Text,
        pictures -> Nullable<Jsonb>,
        uuid -> Uuid,
    }
}

diesel::table! {
    settings (id) {
        id -> Int4,
        profile -> Nullable<Varchar>,
        theme -> Nullable<Varchar>,
        db_path -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    mansionees,
    settings,
);
