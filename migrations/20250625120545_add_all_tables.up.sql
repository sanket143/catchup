create table if not exists user (
    id integer primary key autoincrement,
    username varchar(256) not null unique,
    level integer not null default 1,
    created_at timestamp default current_timestamp,
    is_deleted boolean default false
);

create table if not exists platform (
    id integer primary key autoincrement,
    -- Possible values: codeforces, leetcode, codechef, atcoder, yukicoder
    uid varchar(256) not null unique,
    name varchar(256) not null
);

create table if not exists problem (
    id integer primary key autoincrement,
    -- Useful when doing problem set sync, do not re-add in the DB if it already exists
    -- Possible values: CF/1122/D2, LC/123, CC/SWAPSTR 
    uid varchar(256) not null unique,
    fk_platform_id varchar(256) not null,
    title text not null,
    url text not null,
    rating integer,
    created_at timestamp default current_timestamp,
    metadata json,

    foreign key (fk_platform_id) references platform(id)
);

create table if not exists problem_tag_group (
    id integer primary key autoincrement,
    name varchar(256) not null unique
);

create table if not exists problem_tag (
    id integer primary key autoincrement,
    uid varchar(256) not null unique,
    fk_problem_tag_group_id integer not null default 1,
    created_at timestamp defualt current_timestamp,
    is_deleted boolean default false,

    foreign key (fk_problem_tag_group_id) references problem_tag_group(id)
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

create table if not exists contest (
    id integer primary key autoincrement,
    name varchar(256) not null default 'Local Contest',
    duration integer not null default 120, -- minutes
    created_on integer not null default (strftime('%s', 'now')),
    started_on integer not null default (strftime('%s', 'now')),
    created_for varchar(256) not null,
    fk_problem_tag_group_id integer not null,
    is_evaluated boolean not null default false,
    is_deleted boolean default false,

    foreign key (created_for) references user(username),
    foreign key (fk_problem_tag_group_id) references problem_tag_group(id)
);

create table if not exists contest_problem_map (
    id integer primary key autoincrement,
    fk_contest_id integer not null,
    fk_problem_id integer not null,
    latest_submission_at integer,
    is_deleted boolean default false,
    is_evaluated boolean default false,
    verdict varchar(256) default 'NOT_ATTEMPTED',

    foreign key (fk_contest_id) references contest(id),
    foreign key (fk_problem_id) references problem(id),

    unique(fk_contest_id, fk_problem_id)
);

create table if not exists contest_problem_level (
    id integer primary key autoincrement,
    level integer not null unique,
    duration integer not null, -- in minutes
    performance integer not null,
    problem_rating_level_1 integer not null,
    problem_rating_level_2 integer not null,
    problem_rating_level_3 integer not null,
    problem_rating_level_4 integer not null
);

-- Seeding initial data
-- Add basic platforms
insert into platform (uid, name) values ('codeforces', 'Codeforces') on conflict (uid) do nothing;

-- Add contest problem level based on
-- https://docs.google.com/spreadsheets/d/1gdD-syEpfy10Vz1f5UAm5eKiV_UAEdG-C4jrouN57bs/view
insert into contest_problem_level (
    level,
    duration,
    performance,
    problem_rating_level_1,
    problem_rating_level_2,
    problem_rating_level_3,
    problem_rating_level_4
) values
    (1, 120, 900, 800, 800, 800, 800),
    (2, 120, 950, 800, 800, 800, 900),
    (3, 120, 1000, 800, 800, 900, 900),
    (4, 120, 1050, 800, 900, 900, 900),
    (5, 120, 1100, 800, 900, 900, 1000),
    (6, 120, 1125, 800, 900, 1000, 1000),
    (7, 120, 1150, 800, 1000, 1000, 1000),
    (8, 120, 1175, 800, 1000, 1000, 1100),
    (9, 120, 1200, 800, 1000, 1100, 1100),
    (10, 120, 1250, 800, 1000, 1100, 1200),
    (11, 120, 1300, 800, 1000, 1200, 1200),
    (12, 120, 1350, 800, 1000, 1200, 1300),
    (13, 120, 1400, 800, 1000, 1200, 1400),
    (14, 120, 1425, 900, 1000, 1200, 1400),
    (15, 120, 1450, 900, 1100, 1200, 1400),
    (16, 120, 1475, 900, 1100, 1300, 1400),
    (17, 120, 1500, 900, 1100, 1300, 1500),
    (18, 120, 1525, 1000, 1100, 1300, 1500),
    (19, 120, 1550, 1000, 1200, 1300, 1500),
    (20, 120, 1575, 1000, 1200, 1400, 1500),
    (21, 120, 1600, 1000, 1200, 1400, 1600),
    (22, 120, 1625, 1100, 1200, 1400, 1600),
    (23, 120, 1650, 1100, 1300, 1400, 1600),
    (24, 120, 1675, 1100, 1300, 1500, 1600),
    (25, 120, 1700, 1100, 1300, 1500, 1700),
    (26, 120, 1725, 1200, 1300, 1500, 1700),
    (27, 120, 1750, 1200, 1400, 1500, 1700),
    (28, 120, 1775, 1200, 1400, 1600, 1700),
    (29, 120, 1800, 1200, 1400, 1600, 1800),
    (30, 120, 1825, 1300, 1400, 1600, 1800),
    (31, 120, 1850, 1300, 1500, 1600, 1800),
    (32, 120, 1875, 1300, 1500, 1700, 1800),
    (33, 120, 1900, 1300, 1500, 1700, 1900),
    (34, 120, 1925, 1400, 1500, 1700, 1900),
    (35, 120, 1950, 1400, 1600, 1700, 1900),
    (36, 120, 1975, 1400, 1600, 1800, 1900),
    (37, 120, 2000, 1400, 1600, 1800, 2000),
    (38, 120, 2025, 1500, 1600, 1800, 2000),
    (39, 120, 2050, 1500, 1700, 1800, 2000),
    (40, 120, 2075, 1500, 1700, 1900, 2000),
    (41, 120, 2100, 1500, 1700, 1900, 2100),
    (42, 120, 2125, 1600, 1700, 1900, 2100),
    (43, 120, 2150, 1600, 1800, 1900, 2100),
    (44, 120, 2175, 1600, 1800, 2000, 2100),
    (45, 120, 2200, 1600, 1800, 2000, 2200),
    (46, 120, 2225, 1700, 1800, 2000, 2200),
    (47, 120, 2250, 1700, 1900, 2000, 2200),
    (48, 120, 2275, 1700, 1900, 2100, 2200),
    (49, 120, 2300, 1700, 1900, 2100, 2300),
    (50, 120, 2325, 1800, 1900, 2100, 2300),
    (51, 120, 2350, 1800, 2000, 2100, 2300),
    (52, 120, 2375, 1800, 2000, 2200, 2300),
    (53, 120, 2400, 1800, 2000, 2200, 2400),
    (54, 120, 2425, 1900, 2000, 2200, 2400),
    (55, 125, 2450, 1900, 2100, 2200, 2400),
    (56, 125, 2475, 1900, 2100, 2300, 2400),
    (57, 130, 2500, 1900, 2100, 2300, 2500),
    (58, 130, 2525, 2000, 2100, 2300, 2500),
    (59, 135, 2550, 2000, 2200, 2300, 2500),
    (60, 135, 2575, 2000, 2200, 2400, 2500),
    (61, 140, 2600, 2000, 2200, 2400, 2600),
    (62, 140, 2625, 2100, 2200, 2400, 2600),
    (63, 145, 2650, 2100, 2300, 2400, 2600),
    (64, 145, 2675, 2100, 2300, 2500, 2600),
    (65, 150, 2700, 2100, 2300, 2500, 2700),
    (66, 150, 2725, 2200, 2300, 2500, 2700),
    (67, 155, 2750, 2200, 2400, 2500, 2700),
    (68, 155, 2775, 2200, 2400, 2600, 2700),
    (69, 160, 2800, 2200, 2400, 2600, 2800),
    (70, 160, 2825, 2300, 2400, 2600, 2800),
    (71, 165, 2850, 2300, 2500, 2600, 2800),
    (72, 165, 2875, 2300, 2500, 2700, 2800),
    (73, 170, 2900, 2300, 2500, 2700, 2900),
    (74, 170, 2925, 2400, 2500, 2700, 2900),
    (75, 175, 2950, 2400, 2600, 2700, 2900),
    (76, 175, 2975, 2400, 2600, 2800, 2900),
    (77, 180, 3000, 2400, 2600, 2800, 3000),
    (78, 180, 3025, 2500, 2600, 2800, 3000),
    (79, 180, 3050, 2500, 2700, 2800, 3000),
    (80, 180, 3075, 2500, 2700, 2900, 3000),
    (81, 180, 3100, 2500, 2700, 2900, 3100),
    (82, 180, 3125, 2600, 2700, 2900, 3100),
    (83, 180, 3150, 2600, 2800, 2900, 3100),
    (84, 180, 3175, 2600, 2800, 3000, 3100),
    (85, 180, 3200, 2600, 2800, 3000, 3200),
    (86, 180, 3225, 2700, 2800, 3000, 3200),
    (87, 180, 3250, 2700, 2900, 3000, 3200),
    (88, 180, 3275, 2700, 2900, 3100, 3200),
    (89, 180, 3300, 2700, 2900, 3100, 3300),
    (90, 180, 3325, 2800, 2900, 3100, 3300),
    (91, 180, 3350, 2800, 3000, 3100, 3300),
    (92, 180, 3375, 2800, 3000, 3200, 3300),
    (93, 180, 3400, 2800, 3000, 3200, 3400),
    (94, 180, 3425, 2900, 3000, 3200, 3400),
    (95, 180, 3450, 2900, 3100, 3200, 3400),
    (96, 180, 3475, 2900, 3100, 3300, 3400),
    (97, 180, 3500, 2900, 3100, 3300, 3500),
    (98, 180, 3550, 3000, 3100, 3300, 3500),
    (99, 180, 3600, 3100, 3100, 3300, 3500),
    (100, 180, 3650, 3100, 3200, 3300, 3500),
    (101, 180, 3700, 3200, 3200, 3300, 3500),
    (102, 180, 3725, 3200, 3300, 3300, 3500),
    (103, 180, 3750, 3300, 3300, 3300, 3500),
    (104, 180, 3775, 3300, 3300, 3400, 3500),
    (105, 180, 3800, 3300, 3400, 3400, 3500),
    (106, 180, 3850, 3400, 3400, 3400, 3500),
    (107, 180, 3900, 3400, 3400, 3500, 3500),
    (108, 180, 3950, 3400, 3500, 3500, 3500),
    (109, 180, 4000, 3500, 3500, 3500, 3500)
on conflict (level) do update set
    duration = EXCLUDED.duration,
    performance = EXCLUDED.performance,
    problem_rating_level_1 = EXCLUDED.problem_rating_level_1,
    problem_rating_level_2 = EXCLUDED.problem_rating_level_3,
    problem_rating_level_3 = EXCLUDED.problem_rating_level_3,
    problem_rating_level_4 = EXCLUDED.problem_rating_level_4;

insert into problem_tag_group (
    name
) values
    ('Implementation'),
    ('DP'),
    ('Graphs'),
    ('Trees'),
    ('Mathematics'),
    ('Sortings'),
    ('Bitmasks'),
    ('Brute Force'),
    ('DSA');

-- Group these as you like
insert into problem_tag (uid, fk_problem_tag_group_id) values
    ('dsu', 1),
    ('flows', 1),
    ('2-sat', 1),
    ('greedy', 1),
    ('strings', 1),
    ('interactive', 1),
    ('two pointers', 1),
    ('number theory', 1),
    ('implementation', 1),
    ('meet-in-the-middle', 1),
    ('string suffix structures', 1),
    ('dp', 2),
    ('matrices', 2),
    ('divide and conquer', 2),
    ('graphs', 3),
    ('shortest paths', 3),
    ('dfs and similar', 3),
    ('graph matchings', 3),
    ('trees', 4),
    ('fft', 5),
    ('math', 5),
    ('probabilities', 5),
    ('combinatorics', 5),
    ('sortings', 6),
    ('schedules', 6),
    ('binary search', 6),
    ('bitmasks', 7),
    ('brute force', 8),
    ('data structures', 9),
    ('constructive algorithms', 9)
on conflict (uid) do update set
    fk_problem_tag_group_id = EXCLUDED.fk_problem_tag_group_id;

