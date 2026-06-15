create table gallery_images (
    id uuid primary key default gen_random_uuid(),
    project_id uuid not null references projects (id) on delete cascade,
    storage_key text not null,
    caption text not null default '',
    content_type text not null,
    position int not null default 0,
    created_at timestamptz not null default now()
);

create index gallery_images_project_id_idx on gallery_images (project_id);
