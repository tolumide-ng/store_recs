-- Your SQL goes here
CREATE TABLE country_code (
    id SERIAL PRIMARY KEY,
    country_code VARCHAR NOT NULL,
    iso_code VARCHAR NOT NULL,
    country VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('country_code');