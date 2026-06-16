create table project_moderator_notes (
    id uuid primary key default gen_random_uuid(),
    project_id uuid not null references projects (id) on delete cascade,
    author_id uuid not null references users (id) on delete cascade,
    body text not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index project_moderator_notes_project_id_idx
    on project_moderator_notes (project_id, created_at desc);
