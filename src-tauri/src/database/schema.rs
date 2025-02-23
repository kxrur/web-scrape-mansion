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
