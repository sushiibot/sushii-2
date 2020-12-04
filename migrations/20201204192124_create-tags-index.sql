CREATE EXTENSION pg_trgm;
CREATE INDEX tag_name_idx ON tags USING GIN(tag_name gin_trgm_ops);
