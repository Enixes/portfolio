CREATE TABLE projects (
                          id SERIAL PRIMARY KEY,
                          title TEXT NOT NULL,
                          description TEXT NOT NULL,
                          route TEXT DEFAULT ''
)