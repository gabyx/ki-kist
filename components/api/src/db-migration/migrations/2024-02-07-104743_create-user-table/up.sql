-- The users table in normal form 3.
-- which stores all users for which keys are in the `kikist`.
CREATE TABLE users (
    -- The unique id of the user.
    id uuid PRIMARY KEY,
    -- The name of the user.
    name text NOT NULL
);
