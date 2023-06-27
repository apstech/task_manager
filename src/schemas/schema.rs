// @generated automatically by Diesel CLI.

diesel::table! {
    teams (team_id) {
        team_id -> Integer,
        #[max_length = 4]
        type_teams -> Varchar,
        #[max_length = 150]
        description -> Varchar,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        #[max_length = 20]
        name_user -> Varchar,
        #[max_length = 50]
        password -> Varchar,
        date_register -> Datetime,
        //date_update -> Option<Datetime>,
        team -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    teams,
    users,
);
