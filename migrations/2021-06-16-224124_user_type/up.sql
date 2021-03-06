-- Your SQL goes here
CREATE TABLE user_type (
  id SERIAL PRIMARY KEY,
  auth_type VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('user_type');

INSERT INTO user_type (auth_type) VALUES('ADMIN');

INSERT INTO user_type (auth_type) VALUES('SUPER_ADMIN');

INSERT INTO user_type (auth_type) VALUES('E_USER');

INSERT INTO user_type (auth_type) VALUES('USER');

INSERT INTO user_type (auth_type) VALUES('STAFF');

INSERT INTO user_type (auth_type) VALUES('SUPER_STAFF');

INSERT INTO user_type (auth_type) VALUES('AUDITOR');

