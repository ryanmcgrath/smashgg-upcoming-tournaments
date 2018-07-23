create table if not exists tournaments (
    id serial primary key,
    tournament_id int not null,
    tournament_type int not null,
    name text not null,
    details text not null,
    location text not null,
    url text not null,
    hashtag text not null,
    games text not null,
    starts timestamp not null,
    ends timestamp not null,
    timezone text not null,
    published boolean not null default true,
    added timestamp with time zone not null default now()
);
