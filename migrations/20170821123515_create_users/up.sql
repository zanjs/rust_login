CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE,
  email VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  password VARCHAR NOT NULL
)
