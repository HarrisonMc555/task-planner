table! {
    tasks (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Text,
        description -> Nullable<Text>,
        due -> Nullable<Timestamptz>,
        complete -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        display_name -> Text,
    }
}

joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(tasks, users,);
