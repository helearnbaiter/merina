-- Add migration script for casbin rules table
CREATE TABLE IF NOT EXISTS casbin_rule (
    id SERIAL PRIMARY KEY,
    ptype VARCHAR(128) NOT NULL,
    v0 VARCHAR(128),
    v1 VARCHAR(128),
    v2 VARCHAR(128),
    v3 VARCHAR(128),
    v4 VARCHAR(128),
    v5 VARCHAR(128)
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_casbin_rule_ptype ON casbin_rule (ptype);
CREATE INDEX IF NOT EXISTS idx_casbin_rule_v0 ON casbin_rule (v0);
CREATE INDEX IF NOT EXISTS idx_casbin_rule_v1 ON casbin_rule (v1);