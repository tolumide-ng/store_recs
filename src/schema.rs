table! {
    user_type (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
    }
}
