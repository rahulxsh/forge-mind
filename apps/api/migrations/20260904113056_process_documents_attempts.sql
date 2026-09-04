-- Add migration script here
ALTER TABLE documents
ADD COLUMN last_error TEXT;

ALTER TABLE documents
ADD COLUMN attempts INTEGER default 0;