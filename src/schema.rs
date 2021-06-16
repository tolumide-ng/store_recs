table! {
    user_type (type_id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        type_id -> Uuid,
    }
}
