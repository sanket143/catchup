create table if not exists user (
    id integer primary key autoincrement,
    username varchar(256) not null,
    created_at timestamp default current_timestamp
);

create table if not exists platform (
    id integer primary key autoincrement,
    -- Possible values: codeforces, leetcode, codechef, atcoder, yukicoder
    uid varchar(256) not null unique,
    name varchar(256) not null
);

-- Add basic platforms
insert into platform (uid, name) values ('codeforces', 'Codeforces') on conflict (uid) do nothing;

create table if not exists problem (
    id integer primary key autoincrement,
    -- Useful when doing problem set sync, do not re-add in the DB if it already exists
    -- Possible values: CF/1122/D2, LC/123, CC/SWAPSTR 
    uid varchar(256) not null unique,
    fk_platform_id varchar(256) not null,
    title text,
    url text not null,
    rating integer,
    created_at timestamp default current_timestamp,
    metadata json,

    foreign key (fk_platform_id) references platform(id)
);

create table if not exists problem_tag (
    id integer primary key autoincrement,
    uid varchar(256) not null unique,
    group_name varchar(256),
    created_at timestamp defualt current_timestamp,
    is_deleted boolean default false
);

create table if not exists problem_tag_map (
    id integer primary key autoincrement,
    fk_problem_id integer not null,
    fk_problem_tag_id integer not null,
    is_deleted boolean default false,

    foreign key (fk_problem_id) references problem(id),
    foreign key (fk_problem_tag_id) references problem_tag(id),

    unique (fk_problem_id, fk_problem_tag_id)
);

create index if not exists problem_tag_map_problem_tag_id_idx on problem_tag_map (fk_problem_tag_id);
