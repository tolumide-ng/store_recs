table! {
    country_code (id) {
        id -> Int4,
        country_phone_code -> Varchar,
        iso_code -> Varchar,
        country -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_info (id) {
        id -> Int4,
        user_id -> Nullable<Uuid>,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        phone -> Nullable<Varchar>,
        phone_code -> Nullable<Int4>,
        password -> Text,
        birth_country -> Varchar,
        residing_country -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_type (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    country_code,
    user_info,
    user_type,
);
