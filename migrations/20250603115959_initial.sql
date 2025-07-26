-- Initial SQL migration for Lerpz auth service

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Organizations

CREATE TABLE IF NOT EXISTS organizations(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(64) NOT NULL UNIQUE,
    description TEXT DEFAULT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON organizations
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- Users

CREATE TABLE IF NOT EXISTS users(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(32) NOT NULL UNIQUE,
    primary_email VARCHAR(64) NOT NULL UNIQUE,
    password_hash VARCHAR(128) NOT NULL,
    password_salt VARCHAR(64) NOT NULL,
    avatar VARCHAR(256) DEFAULT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    organization_id UUID DEFAULT NULL REFERENCES organizations(id)
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- Client & Scopes 

CREATE TABLE IF NOT EXISTS clients(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(64) NOT NULL UNIQUE,
    description TEXT DEFAULT NULL,
<<<<<<< HEAD
    organization_id UUID NOT NULL REFERENCES organizations(id),
||||||| f15d1ea
    organization_id UUID DEFAULT NULL REFERENCES organizations(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
=======
    organization_id UUID DEFAULT NULL REFERENCES organizations(id),
>>>>>>> oauth
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON clients
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

<<<<<<< HEAD
||||||| f15d1ea
-- User Consents

CREATE TABLE IF NOT EXISTS user_client_authorizations(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    client_id UUID NOT NULL REFERENCES oauth_clients(id),
    scope TEXT NOT NULL,
    authorized_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP DEFAULT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    UNIQUE (user_id, client_id)
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON user_client_authorizations
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- OAuth Redirect URIs

CREATE TABLE IF NOT EXISTS redirect_uris(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    client_id UUID NOT NULL REFERENCES oauth_clients(id),
    uri VARCHAR(512) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON redirect_uris
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- OAuth Scopes

=======
-- User Consents

CREATE TABLE IF NOT EXISTS user_client_consent(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    client_id UUID NOT NULL REFERENCES oauth_clients(id),
    authorized_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    UNIQUE (user_id, client_id)
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON user_client_authorizations
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- OAuth Redirect URIs

CREATE TABLE IF NOT EXISTS redirect_uris(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    uri VARCHAR(2000) NOT NULL,
    client_id UUID NOT NULL REFERENCES oauth_clients(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON redirect_uris
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- OAuth Scopes

>>>>>>> oauth
CREATE TABLE IF NOT EXISTS scopes(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(64) NOT NULL UNIQUE,
    description TEXT,
    parent_scope_id UUID DEFAULT NULL REFERENCES scopes(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON scopes
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

CREATE TABLE IF NOT EXISTS client_scopes(
    scope_id UUID NOT NULL REFERENCES scopes(id),
<<<<<<< HEAD
    client_id UUID NOT NULL REFERENCES clients(id),
||||||| f15d1ea
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
=======
>>>>>>> oauth
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (client_id, scope_id)
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON client_scopes
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

<<<<<<< HEAD
||||||| f15d1ea
-- Access Tokens

CREATE TABLE IF NOT EXISTS access_tokens(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    jti VARCHAR(128) NOT NULL UNIQUE,
    user_id UUID DEFAULT NULL REFERENCES users(id),
    client_id UUID NOT NULL REFERENCES oauth_clients(id),
    scope TEXT,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    revoked_at TIMESTAMP DEFAULT NULL
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON access_tokens
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

=======
-- Access Tokens

CREATE TABLE IF NOT EXISTS access_tokens(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    jti VARCHAR(128) NOT NULL UNIQUE,
    user_id UUID DEFAULT NULL REFERENCES users(id),
    client_id UUID NOT NULL REFERENCES oauth_clients(id),
    scope TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    revoked_at TIMESTAMPTZ DEFAULT NULL
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON access_tokens
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

>>>>>>> oauth
-- Refresh Tokens

CREATE TABLE IF NOT EXISTS refresh_tokens(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    token VARCHAR(512) NOT NULL UNIQUE,
    user_id UUID NOT NULL REFERENCES users(id),
<<<<<<< HEAD
    client_id UUID NOT NULL REFERENCES clients(id),
||||||| f15d1ea
    client_id UUID NOT NULL REFERENCES oauth_clients(id),
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    revoked_at TIMESTAMP DEFAULT NULL
=======
    client_id UUID NOT NULL REFERENCES oauth_clients(id),
>>>>>>> oauth
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    revoked_at TIMESTAMPTZ DEFAULT NULL
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON refresh_tokens
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- Seeding of Lerpz client & scopes

<<<<<<< HEAD
INSERT INTO
    organizations (name, description)
VALUES
    ('Lerpz', 'Lerpz organization'),
    ('Lerpz Client', 'Client for Lerpz organization');

INSERT INTO
    clients (name, description, organization_id)
VALUES
(
    'Lerpz Client',
    'Client for Lerpz organization',
    (SELECT id FROM organizations WHERE name = 'Lerpz')
);
||||||| f15d1ea
INSERT INTO oauth_clients(
    id,
    secret,
    name,
    description
) VALUES (
    'cdd37e5a-a554-4535-bff2-45ba130b05b4',
    'todo-add-secret-string',
    'Lerpz Portal',
    'The main client for the Lerpz portal.'
)
=======
INSERT INTO oauth_clients(
    id,
    name,
    description
) VALUES (
    'cdd37e5a-a554-4535-bff2-45ba130b05b4',
    'Lerpz Portal',
    'The main client for the Lerpz portal.'
)
>>>>>>> oauth
