create table project_members (
    id uuid primary key default gen_random_uuid(),
    project_id uuid not null references projects (id) on delete cascade,
    user_id uuid not null references users (id) on delete cascade,
    role text not null default 'member',
    created_at timestamptz not null default now(),

    unique (project_id, user_id)
);

create index project_members_user_id_idx on project_members (user_id);
