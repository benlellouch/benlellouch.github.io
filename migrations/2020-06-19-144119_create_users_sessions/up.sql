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

CREATE TABLE languages
(
  id SERIAL NOT NULL PRIMARY KEY,
  language TEXT NOT NULL,
  proficiency TEXT NOT NULL
);

CREATE TABLE profile
(
  id SERIAL NOT NULL PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  profile_path TEXT NOT NULL, 
  title TEXT NOT NULL,
  location TEXT NOT NULL,
  email TEXT NOT NULL,
  about_me TEXT NOT NULL,
  github_link TEXT NOT NULL,
  linkedin_link TEXT NOT NULL
);

INSERT INTO profile 
(
  first_name,
  last_name,
  profile_path,
  title,
  location,
  email,
  about_me,
  github_link,
  linkedin_link
)
VALUES
(
  'John',
  'Appleseed',
  'assets/images/profile/default.jpg',
  'Software Developer',
  'Los Angeles, CA',
  'john@appleseed.com',
  'Lorem Ipsum',
  'https://github.com',
  'https://linkedin.com'
);