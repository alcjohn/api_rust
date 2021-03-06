table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
