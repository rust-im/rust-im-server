CREATE ROLE rustim WITH LOGIN NOSUPERUSER INHERIT CREATEDB NOCREATEROLE NOREPLICATION PASSWORD 'rustim';
CREATE DATABASE rustim;
GRANT ALL PRIVILEGES ON DATABASE rustim TO rustim;
