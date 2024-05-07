CREATE TABLE user_account (
	id SERIAL PRIMARY KEY,
	username TEXT NOT NULL,
	email TEXT NOT NULL,
	password TEXT NOT NULL
)
