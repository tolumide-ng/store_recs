-- Your SQL goes here
CREATE TABLE country_code (
    id SERIAL PRIMARY KEY,
    country_code VARCHAR NOT NULL,
    iso_code VARCHAR NOT NULL,
    country VARCHAR NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
)