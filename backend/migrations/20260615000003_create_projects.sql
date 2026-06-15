create table projects (
    id uuid primary key default gen_random_uuid(),
    slug text not null unique,
    title text not null,
    summary text not null default '',
    description text not null default '',
    project_type text not null,
    owner_id uuid not null references users (id) on delete cascade,
    download_count bigint not null default 0,
    published boolean not null default false,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index projects_owner_id_idx on projects (owner_id);
create index projects_project_type_idx on projects (project_type);
