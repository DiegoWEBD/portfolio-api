create table if not exists project (
    id serial primary key,
    name text not null unique,
    description text not null unique
);