CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    uid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_name VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL, -- Ensure to hash before storing
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_admin BOOLEAN DEFAULT FALSE,
    last_login TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN DEFAULT TRUE
);

-- CREATE TABLE rooms (
--    rid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
--    room_name VARCHAR(50) NOT NULL,
--    owner_id UUID REFERENCES users(uid),
--    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
--); 

--CREATE TABLE room_member (
--    rid UUID REFERENCES rooms(rid),
--    uid UUID REFERENCES users(uid),
--    is_mod BOOLEAN DEFAULT FALSE,
--    active BOOLEAN DEFAULT TRUE,
--    joined_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
--    PRIMARY KEY (rid, uid)
--);

