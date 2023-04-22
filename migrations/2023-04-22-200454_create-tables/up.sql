-- Your SQL goes here

CREATE TABLE users (
  id SERIAL,
  user_id VARCHAR(255) PRIMARY KEY,
  username VARCHAR(255) UNIQUE NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  authenticated BOOLEAN NOT NULL DEFAULT False,
  created_at TIMESTAMP NOT NULL DEFAULT now()::TIMESTAMP
);

CREATE TABLE animes (
  id SERIAL,
  anime_id VARCHAR(255) PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  tracking_data JSONB,
  completed BOOLEAN NOT NULL DEFAULT false
);


CREATE TABLE user_followed_animes (
  id SERIAL,
  user_id VARCHAR(255) NOT NULL,
  anime_id VARCHAR(255) NOT NULL,
  PRIMARY KEY (user_id, anime_id)
);
