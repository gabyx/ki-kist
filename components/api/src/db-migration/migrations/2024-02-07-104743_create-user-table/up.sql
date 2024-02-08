-- The users table in normal form 3.
-- which stores all users for which keys are in the `kikist`.
CREATE TABLE users (
    -- The unique id of the user.
    -- TODO: Should be probably an UUID later for better security.
    id varchar PRIMARY KEY
);
