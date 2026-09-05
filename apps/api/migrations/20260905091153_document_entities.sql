-- Add migration script here
CREATE TABLE IF NOT EXISTS entities (
    id UUID PRIMARY KEY ,
    canonical_name TEXT NOT NULL ,
    entity_type TEXT NOT NULL ,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);


CREATE TABLE IF NOT EXISTS document_entities (
    document_id UUID NOT NULL ,
    entity_id UUID NOT NULL ,
    PRIMARY KEY (document_id, entity_id) ,

    FOREIGN KEY (document_id) REFERENCES documents(id),
    FOREIGN KEY (entity_id) REFERENCES entities(id)
);

CREATE INDEX IF NOT EXISTS idx_documents_entities_entity_id
ON document_entities(entity_id);