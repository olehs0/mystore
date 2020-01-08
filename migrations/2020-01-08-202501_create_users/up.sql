-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(100) NOT NULL,
  username VARCHAR(100) NOT NULL,
  password VARCHAR(64) NOT NULL,
  created_at TIMESTAMP NOT NULL,
  CHECK (email <> '')
);
CREATE INDEX users_email_username_idx ON users (email, username);
