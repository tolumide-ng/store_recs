-- Your SQL goes here
CREATE TABLE user_type (
  id SERIAL,
  type VARCHAR NOT NULL,
  type_id UUID NOT NULL PRIMARY KEY
)