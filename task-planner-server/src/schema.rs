table! {
    plans (id) {
        id -> Int4,
        task_id -> Int4,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        due -> Timestamptz,
        complete -> Bool,
    }
}

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

joinable!(plans -> tasks (task_id));
joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(
    plans,
    tasks,
    users,
);
