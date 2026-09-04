-- Add migration script here
ALTER TABLE documents
ALTER COLUMN attempts SET NOT NULL ;