create table categories (
    id uuid primary key default gen_random_uuid(),
    slug text not null,
    name text not null,
    project_type text not null,
    ordering int not null default 0,

    unique (project_type, slug)
);

create table project_categories (
    project_id uuid not null references projects (id) on delete cascade,
    category_id uuid not null references categories (id) on delete cascade,

    primary key (project_id, category_id)
);

create index project_categories_category_id_idx on project_categories (category_id);

insert into categories (slug, name, project_type, ordering) values
    ('adventure', 'Adventure', 'addon', 1),
    ('utility', 'Utility', 'addon', 2),
    ('mobs', 'Mobs', 'addon', 3),
    ('equipment', 'Equipment', 'addon', 4),
    ('magic', 'Magic', 'addon', 5),
    ('technology', 'Technology', 'addon', 6),
    ('decoration', 'Decoration', 'addon', 7),
    ('food', 'Food', 'addon', 8),

    ('survival', 'Survival', 'world', 1),
    ('adventure', 'Adventure', 'world', 2),
    ('creative', 'Creative', 'world', 3),
    ('minigame', 'Minigame', 'world', 4),
    ('parkour', 'Parkour', 'world', 5),
    ('puzzle', 'Puzzle', 'world', 6),
    ('roleplay', 'Roleplay', 'world', 7),

    ('realistic', 'Realistic', 'resource_pack', 1),
    ('simplistic', 'Simplistic', 'resource_pack', 2),
    ('themed', 'Themed', 'resource_pack', 3),
    ('ui', 'UI', 'resource_pack', 4),
    ('audio', 'Audio', 'resource_pack', 5),

    ('characters', 'Characters', 'skin_pack', 1),
    ('animals', 'Animals', 'skin_pack', 2),
    ('fantasy', 'Fantasy', 'skin_pack', 3),
    ('sci-fi', 'Sci-Fi', 'skin_pack', 4),
    ('seasonal', 'Seasonal', 'skin_pack', 5);
