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

CREATE TABLE skills
(
  id SERIAL NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  origin TEXT NOT NULL,
  yoxp INTEGER NOT NULL
);

CREATE TABLE about_me
(
  id SERIAL NOT NULL PRIMARY KEY,
  description TEXT NOT NULL
);

CREATE TABLE experience
(
  id SERIAL NOT NULL PRIMARY KEY,
  title TEXT NOT NULL,
  company TEXT NOT NULL,
  year TEXT NOT NULL,
  description TEXT NOT NULL,
  org_link TEXT NOT NULL
);

CREATE TABLE education
(
  id SERIAL NOT NULL PRIMARY KEY,
  major TEXT NOT NULL,
  institution TEXT NOT NULL,
  year TEXT NOT NULL
);