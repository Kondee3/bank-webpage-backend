-- Your SQL goes here
CREATE TABLE bank_users(
    id SERIAL PRIMARY KEY,
    user_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    date_of_birth DATE,
    age INTEGER
)
