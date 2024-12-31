-- Your SQL goes here
CREATE TABLE images (
  id SERIAL PRIMARY KEY,
  image_path TEXT NOT NULL,
  mansion_id INTEGER NOT NULL REFERENCES mansions(id)
);
