// @generated automatically by Diesel CLI.

diesel::table! {
    keys (id) {
        id -> Uuid,
        user_id -> Varchar,
        public_key -> Text,
        private_key_encrypted -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
    }
}

diesel::joinable!(keys -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    keys,
    users,
);
