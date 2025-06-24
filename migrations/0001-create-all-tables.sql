create table if not exists user (
    id integer primary key autoincrement,
    username varchar(256) not null,
    created_at timestamp default current_timestamp
);

create table if not exists platform (
    id integer primary key autoincrement,
    uid varchar(256) not null, -- codeforces, leetcode, codechef, atcoder, yukicoder
    name varchar(256) not null
);

create table if not exists problem (
    id integer primary key autoincrement,
    fk_platform_uid varchar(256) not null,
    problem_uid varchar(256) not null, -- CF/1122/D2, LC/123
    title text,
    url text not null,
    created_at timestamp default current_timestamp,
    metadata text,

    foreign key (fk_platform_uid) references platform(uid)
);
