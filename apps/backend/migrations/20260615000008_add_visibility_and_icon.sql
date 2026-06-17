alter table projects
    add column visibility text not null default 'public'
        check (visibility in ('public', 'unlisted', 'private')),
    add column icon_key text;
