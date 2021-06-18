-- Your SQL goes here
CREATE TABLE user_info (
    id SERIAL PRIMARY KEY,
    user_id uuid DEFAULT uuid_generate_v4(),
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    phone VARCHAR,
    phone_code INTEGER,
    FOREIGN KEY (phone_code) REFERENCES country_code,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
    -- implement the country, state/location with neo4j
)

