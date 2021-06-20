-- Your SQL goes here
CREATE TABLE country_code (
    id SERIAL PRIMARY KEY,
    country_phone_code VARCHAR UNIQUE NOT NULL,
    iso_code VARCHAR UNIQUE NOT NULL,
    country VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('country_code');