-- Drop indexes
DROP INDEX IF EXISTS idx_users_username;
DROP INDEX IF EXISTS idx_users_email;
DROP INDEX IF EXISTS idx_data_sources_name;
DROP INDEX IF EXISTS idx_casbin_rules_ptype;

-- Drop tables
DROP TABLE IF EXISTS casbin_rules;
DROP TABLE IF EXISTS data_sources;
DROP TABLE IF EXISTS users;