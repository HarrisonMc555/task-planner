CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT UNIQUE NOT NULL UNIQUE CHECK (username <> ''),
  display_name TEXT NOT NULL
)
