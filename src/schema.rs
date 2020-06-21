table! {
    projects (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
        link -> Text,
        img_path -> Text,
        is_primary -> Bool,
    }
}

table! {
    users (userid) {
        userid -> Oid,
        username -> Varchar,
        display -> Nullable<Varchar>,
        is_admin -> Bool,
        salt_hash -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    projects,
    users,
);
