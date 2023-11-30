-- Add migration script here
create table todos (
    id serial primary key,
    description text not null
);