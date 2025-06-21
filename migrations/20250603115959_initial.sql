CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS organizations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(64) NOT NULL UNIQUE,
    description TEXT DEFAULT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON organizations
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(32) NOT NULL UNIQUE,
    primary_email VARCHAR(64) NOT NULL UNIQUE,
    password_hash VARCHAR(128) NOT NULL,
    password_salt VARCHAR(64) NOT NULL,
    avatar VARCHAR(256) DEFAULT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    organization_id UUID DEFAULT NULL REFERENCES organizations(id)
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

CREATE TABLE IF NOT EXISTS oauth_clients (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    secret VARCHAR(512) NOT NULL,
    name VARCHAR(64) NOT NULL UNIQUE,
    description TEXT DEFAULT NULL,
    organization_id UUID DEFAULT NULL REFERENCES organizations(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON oauth_clients
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

CREATE TABLE IF NOT EXISTS redirect_uris (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    client_id UUID NOT NULL REFERENCES oauth_clients(id),
    uri VARCHAR(512) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON redirect_uris
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

INSERT INTO oauth_clients (id, secret, name, description) VALUES (
    'cdd37e5a-a554-4535-bff2-45ba130b05b4',
    'secret-string',
    'Lerpz Portal',
    'The main client for the Lerpz portal.'
)
