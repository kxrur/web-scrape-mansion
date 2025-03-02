-- Your SQL goes here
CREATE TABLE settings (
    id SERIAL PRIMARY KEY,
    profile VARCHAR NOT NULL,
    theme VARCHAR NOT NULL,
    db_path VARCHAR NOT NULL
);
