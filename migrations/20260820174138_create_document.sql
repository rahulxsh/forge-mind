-- Add migration script here
CREATE TYPE document_status AS ENUM (
    'queued',
    'processing',
    'processed',
    'failed'
);

CREATE TABLE documents (
    id UUID PRIMARY KEY ,
    file_name TEXT NOT NULL ,
    content_type TEXT NOT NULL ,
    status document_status NOT NULL ,
    created_at TIMESTAMP NOT NULL DEFAULT NOW() ,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)