table! {
    experiences (id) {
        id -> Int4,
        title -> Text,
        company -> Text,
        description -> Text,
        year -> Text,
        org_link -> Text,
    }
}

table! {
    projects (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
        link -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    experiences,
    projects,
);
