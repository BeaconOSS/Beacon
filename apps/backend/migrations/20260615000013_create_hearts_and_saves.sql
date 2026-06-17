create table project_hearts (
    project_id uuid not null references projects (id) on delete cascade,
    user_id uuid not null references users (id) on delete cascade,
    created_at timestamptz not null default now(),

    primary key (project_id, user_id)
);

create index project_hearts_user_idx on project_hearts (user_id);

create table project_saves (
    project_id uuid not null references projects (id) on delete cascade,
    user_id uuid not null references users (id) on delete cascade,
    created_at timestamptz not null default now(),

    primary key (project_id, user_id)
);

create index project_saves_user_idx on project_saves (user_id);
