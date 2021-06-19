-- Your SQL goes here
CREATE TABLE user_info (
    id SERIAL PRIMARY KEY,
    user_id uuid DEFAULT uuid_generate_v4(),
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    phone VARCHAR,
    phone_code INTEGER,
    password: TEXT NOT NULL,
    FOREIGN KEY (phone_code) REFERENCES country_code,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    -- implement the country, state/location with neo4j
);

SELECT diesel_manage_updated_at('user_info');