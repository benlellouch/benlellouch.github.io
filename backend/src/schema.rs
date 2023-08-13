// @generated automatically by Diesel CLI.

diesel::table! {
    experiences (id) {
        id -> Int4,
        title -> Text,
        company -> Text,
        description -> Text,
        start_date -> Date,
        end_date -> Nullable<Date>,
        org_link -> Text,
    }
}

diesel::table! {
    profiles (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        biography -> Nullable<Text>,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
        link -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    experiences,
    profiles,
    projects,
);
