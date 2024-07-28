PRAGMA foreign_keys = ON;

create table if not exists notes (
	id 		TEXT NOT NULL PRIMARY KEY,
	title 	TEXT,
	note	TEXT
);
