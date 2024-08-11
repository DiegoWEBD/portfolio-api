create table if not exists project (
    id serial primary key,
    name text not null unique,
    description text not null unique,
    image_name text not null unique
);

create table if not exists document (
    id serial primary key,
    title text not null unique,
    description text not null unique,
    download_url text not null unique
);