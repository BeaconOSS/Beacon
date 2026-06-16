create table version_file_analysis (
    file_id uuid primary key references files (id) on delete cascade,
    report jsonb,
    file_index jsonb,
    mctools_version text not null default '',
    schema_version int not null default 1,
    analyzed_at timestamptz not null default now(),
    truncated boolean not null default false,
    error text not null default ''
);
