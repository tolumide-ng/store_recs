-- Your SQL goes here
CREATE TABLE user_info (
    id SERIAL PRIMARY KEY,
    user_id uuid DEFAULT uuid_generate_v4(),
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    phone VARCHAR NOT NULL,
    phone_code INTEGER NOT NULL,
    FOREIGN KEY (phone_code) REFERENCES country_code
    -- implement the country, state/location with neo4j
)

