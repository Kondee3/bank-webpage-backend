-- Your SQL goes here
CREATE TABLE mails(
id VARCHAR PRIMARY KEY,
sender_email VARCHAR NOT NULL,
receiver_email VARCHAR NOT NULL,
time_sent TIMESTAMP NOT NULL,
title VARCHAR NOT NULL,
content VARCHAR NOT NULL
)
