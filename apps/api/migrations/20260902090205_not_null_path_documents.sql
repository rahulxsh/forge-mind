-- Add migration script here
ALTER TABLE documents
ALTER COLUMN path SET NOT NULL;