-- Your SQL goes here

ALTER TABLE bank_users 
ADD CONSTRAINT unique_email UNIQUE (email);


