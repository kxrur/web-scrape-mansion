-- Your SQL goes here
CREATE TABLE mansions (
  id SERIAL PRIMARY KEY,
  address TEXT NOT NULL,
  price TEXT NOT NULL,
  size TEXT NOT NULL,
  bedrooms INT NOT NULL,
  bathrooms INT NOT NULL,
  receptions INT NOT NULL,
  type TEXT NOT NULL
);
