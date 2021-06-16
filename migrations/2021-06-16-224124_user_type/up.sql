-- Your SQL goes here
CREATE TABLE user_type (
  id SERIAL PRIMARY KEY,
  type VARCHAR NOT NULL
);

INSERT INTO user_type (type) VALUES('ADMIN');

INSERT INTO user_type (type) VALUES('SUPER_ADMIN');

INSERT INTO user_type (type) VALUES('E_USER');

INSERT INTO user_type (type) VALUES('USER');

INSERT INTO user_type (type) VALUES('STAFF');

INSERT INTO user_type (type) VALUES('SUPER_STAFF');

INSERT INTO user_type (type) VALUES('AUDITOR');

