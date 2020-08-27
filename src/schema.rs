table! {
    projects (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
    }
}

table! {
    project_users (id) {
        id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        project_id -> Unsigned<Bigint>,
        view_role -> Bool,
        modify_role -> Bool,
        admin_role -> Bool,
    }
}

table! {
    users (id) {
        id -> Unsigned<Bigint>,
        username -> Varchar,
        pw_hash -> Varchar,
    }
}

joinable!(project_users -> projects (project_id));
joinable!(project_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    projects,
    project_users,
    users,
);
