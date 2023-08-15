-- Your SQL goes here

ALTER TABLE bank_users 
ALTER COLUMN dateofbirth TYPE DATE using (dateofbirth::text::date);


