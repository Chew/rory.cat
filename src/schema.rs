// @generated automatically by Diesel CLI.

diesel::table! {
    images (id) {
        id -> Integer,
        url -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    images,
);
