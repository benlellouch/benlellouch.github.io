table! {
    about_me (id) {
        id -> Int4,
        description -> Text,
    }
}

table! {
    education (id) {
        id -> Int4,
        major -> Text,
        institution -> Text,
        year -> Text,
    }
}

table! {
    experience (id) {
        id -> Int4,
        title -> Text,
        company -> Text,
        year -> Text,
        description -> Text,
        org_link -> Text,
    }
}

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
    skills (id) {
        id -> Int4,
        name -> Text,
        origin -> Text,
        yoxp -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    about_me,
    education,
    experience,
    projects,
    skills,
);
