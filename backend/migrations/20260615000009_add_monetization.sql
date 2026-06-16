alter table projects
    add column monetization_enabled boolean not null default true,
    add column creator_share integer not null default 80
        check (creator_share between 0 and 80);
