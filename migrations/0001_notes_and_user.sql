-- Add migration script here

CREATE TABLE IF NOT EXISTS users (
	username		TEXT NOT NULL UNIQUE PRIMARY KEY,
	password_hash	TEXT NOT NULL
);

CREATE TABLE notes (
	id 		TEXT NOT NULL PRIMARY KEY,
	title 	TEXT,
	note	TEXT,
	user_id TEXT NOT NULL,
	FOREIGN KEY (user_id)
	REFERENCES users(username)
		ON DELETE CASCADE
);
