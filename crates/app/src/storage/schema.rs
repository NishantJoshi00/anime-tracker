// @generated automatically by Diesel CLI.

diesel::table! {
    animes (anime_id) {
        id -> Int4,
        anime_id -> Varchar,
        name -> Varchar,
        tracking_data -> Nullable<Jsonb>,
        completed -> Bool,
    }
}

diesel::table! {
    user_followed_animes (user_id, anime_id) {
        id -> Int4,
        user_id -> Varchar,
        anime_id -> Varchar,
    }
}

diesel::table! {
    users (user_id) {
        id -> Int4,
        user_id -> Varchar,
        username -> Varchar,
        email -> Varchar,
        authenticated -> Bool,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    animes,
    user_followed_animes,
    users,
);
