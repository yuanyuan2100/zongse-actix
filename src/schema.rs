table! {
    comments (id) {
        id -> Int4,
        body -> Text,
        create_time -> Timestamp,
        comment_by -> Varchar,
        comment_id -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        id_url -> Varchar,
        title -> Varchar,
        subtitle -> Varchar,
        body -> Text,
        published -> Bool,
        create_time -> Timestamp,
        view -> Int4,
        tags -> Array<Text>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        create_at -> Timestamp,
        last_login_at -> Timestamp,
    }
}

joinable!(comments -> posts (comment_id));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    users,
);
