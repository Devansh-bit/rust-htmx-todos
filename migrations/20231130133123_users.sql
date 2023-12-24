-- Add migration script here
CREATE TABLE users (

    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

    username text NOT NULL UNIQUE,

    password text NOT NULL

);
