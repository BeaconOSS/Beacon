alter table projects
    add column website_url text not null default '',
    add column source_url text not null default '',
    add column issues_url text not null default '',
    add column wiki_url text not null default '',
    add column discord_url text not null default '';
