-- Your SQL goes here
CREATE TABLE pictures (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    mansionee_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    FOREIGN KEY (mansionee_id) REFERENCES mansionees(id) ON DELETE CASCADE
);

CREATE INDEX idx_pictures_mansionees_id ON pictures (mansionee_id);
