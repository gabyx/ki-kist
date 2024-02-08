-- The keys table in normal form 3.
-- which stores all assymmetric keys.
CREATE TABLE keys (
    -- The unique id of the assymetric key pair.
    id uuid PRIMARY KEY,
    --
    -- The user_id this key belongs to.
    user_id varchar NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (
        id
    ) ON DELETE CASCADE,
    --
    -- The assymetric keys.
    public_key text NOT NULL,
    private_key_encrypted text NOT NULL
);
