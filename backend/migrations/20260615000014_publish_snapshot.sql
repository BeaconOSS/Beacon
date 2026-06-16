alter table projects
    add column published_title text,
    add column published_summary text,
    add column published_description text,
    add column published_icon_key text,
    add column published_license text not null default '',
    add column published_at timestamptz,
    add column pending_changelog text not null default '';

create table project_published_categories (
    project_id uuid not null references projects (id) on delete cascade,
    category_id uuid not null references categories (id) on delete cascade,
    primary key (project_id, category_id)
);

update projects
set
    published_title = title,
    published_summary = summary,
    published_description = description,
    published_icon_key = icon_key,
    published_license = license,
    published_at = coalesce(updated_at, now())
where status = 'approved';

insert into project_published_categories (project_id, category_id)
select pc.project_id, pc.category_id
from project_categories pc
join projects p on p.id = pc.project_id
where p.status = 'approved';
