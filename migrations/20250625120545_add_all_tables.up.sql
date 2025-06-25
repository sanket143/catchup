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
    platform_uid varchar(256) not null unique,
    fk_platform_id varchar(256) not null,
    title text,
    url text not null,
    created_at timestamp default current_timestamp,
    metadata json,

    foreign key (fk_platform_id) references platform(id)
);
