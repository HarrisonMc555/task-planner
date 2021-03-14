CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT UNIQUE NOT NULL CHECK (username <> ''),
  display_name TEXT NOT NULL
)
