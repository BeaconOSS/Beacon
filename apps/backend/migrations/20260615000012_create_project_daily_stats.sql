create table project_daily_stats (
    project_id uuid not null references projects (id) on delete cascade,
    day date not null default (now() at time zone 'utc')::date,
    views integer not null default 0,
    downloads integer not null default 0,

    primary key (project_id, day)
);

create index project_daily_stats_day_idx on project_daily_stats (day);
