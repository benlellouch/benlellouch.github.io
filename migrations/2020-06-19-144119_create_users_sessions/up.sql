-- Your SQL goes here 
CREATE TABLE projects
(
    id SERIAL NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    link TEXT NOT NULL,
    img_path TEXT NOT NULL,
    is_primary BOOLEAN NOT NULL DEFAULT 'f'
);