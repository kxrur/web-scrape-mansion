-- Your SQL goes here
CREATE TABLE mansionees (
    id SERIAL PRIMARY KEY,
    address TEXT NOT NULL,
    price INTEGER,
    size DOUBLE PRECISION,
    bedrooms INTEGER,
    bathrooms INTEGER,
    receptions INTEGER,
    house_type TEXT NOT NULL
);
