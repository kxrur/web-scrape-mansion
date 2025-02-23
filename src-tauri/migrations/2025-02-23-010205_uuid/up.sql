-- Your SQL goes here
ALTER TABLE mansionees ADD COLUMN uuid UUID DEFAULT gen_random_uuid() NOT NULL;
