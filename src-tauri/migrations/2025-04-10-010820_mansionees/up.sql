-- Your SQL goes here
CREATE TABLE mansionees (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    address TEXT NOT NULL,
    price INTEGER,
    size REAL,
    bedrooms INTEGER,
    bathrooms INTEGER,
    receptions INTEGER,
    house_type TEXT NOT NULL,
    uuid BINARY NOT NULL UNIQUE
);
