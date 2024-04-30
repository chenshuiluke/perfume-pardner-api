CREATE TABLE perfume (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL,
	designer TEXT NOT NULL,
	release_year INTEGER NOT NULL,
	thumbnail TEXT NOT NULL,
	fragrantica_id INTEGER NOT NULL,
	fragrantica_url TEXT NOT NULL
)
