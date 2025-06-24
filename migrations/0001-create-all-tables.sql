create table if not exists user (
    id integer primary key autoincrement,
    username varchar(256) not null,
    created_at timestamp default current_timestamp
);

create table if not exists platform (
    id integer primary key autoincrement,
    uid varchar(256) not null unique, -- codeforces, leetcode, codechef, atcoder, yukicoder
    name varchar(256) not null
);

insert into platform (uid, name) values ('codeforces', 'Codeforces');

create table if not exists problem (
    id integer primary key autoincrement,
    platform_uid varchar(256) not null unique, -- CF/1122/D2, LC/123
    fk_platform_id varchar(256) not null,
    title text,
    url text not null,
    created_at timestamp default current_timestamp,
    metadata json,

    foreign key (fk_platform_id) references platform(id)
);
