create table versions (
    id uuid primary key default gen_random_uuid(),
    project_id uuid not null references projects (id) on delete cascade,
    version_number text not null,
    name text not null default '',
    changelog text not null default '',
    channel text not null default 'release',
    download_count bigint not null default 0,
    created_at timestamptz not null default now(),

    unique (project_id, version_number)
);

create index versions_project_id_idx on versions (project_id);

create table files (
    id uuid primary key default gen_random_uuid(),
    version_id uuid not null references versions (id) on delete cascade,
    filename text not null,
    size bigint not null,
    sha256 text not null,
    storage_key text not null,
    is_primary boolean not null default true,
    created_at timestamptz not null default now()
);

create index files_version_id_idx on files (version_id);
