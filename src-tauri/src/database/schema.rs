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
    }
}
