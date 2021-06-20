-- Your SQL goes here
CREATE TABLE user_info (
    id SERIAL PRIMARY KEY,
    user_id uuid DEFAULT uuid_generate_v4(),
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    phone VARCHAR,
    phone_code INTEGER,
    password TEXT NOT NULL,
    -- phone_code VARCHAR 
    birth_country VARCHAR NOT NULL,
    residing_country VARCHAR NOT NULL,
    FOREIGN KEY (birth_country) REFERENCES country_code(iso_code) ON UPDATE CASCADE,
    FOREIGN KEY (residing_country) REFERENCES country_code(iso_code) ON UPDATE CASCADE,
    -- FOREIGN KEY (phone_code) REFERENCES country_code(code) ON UPDATE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    -- implement the country, state/location with neo4j
);

SELECT diesel_manage_updated_at('user_info');