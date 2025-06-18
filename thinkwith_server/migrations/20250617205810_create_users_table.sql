-- Add migration script here
create table if not exists users (
    id serial primary key,
    username varchar(50) not null unique,
    password_hash varchar(255) not null,
    email varchar(100) not null unique,
    created_at timestamp with time zone default current_timestamp,
    updated_at timestamp with time zone default current_timestamp
);