alter table users
    add column role text not null default 'user'
        check (role in ('user', 'moderator', 'admin'));

alter table projects
    add column status text not null default 'draft'
        check (
            status in (
                'draft',
                'in_review',
                'changes_requested',
                'approved',
                'rejected'
            )
        ),
    add column license text not null default '',
    add column submitted_at timestamptz;

update projects set status = 'draft';

alter table projects drop column published;

create index projects_status_idx on projects (status);

create table project_reviews (
    id uuid primary key default gen_random_uuid(),
    project_id uuid not null references projects (id) on delete cascade,
    reviewer_id uuid not null references users (id) on delete cascade,
    action text not null check (
        action in ('approve', 'reject', 'request_changes')
    ),
    notes text not null default '',
    created_at timestamptz not null default now()
);

create index project_reviews_project_id_idx
    on project_reviews (project_id, created_at desc);
