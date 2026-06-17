create table users (
    id uuid primary key default gen_random_uuid(),
    username text not null unique,
    email text not null unique,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table user_identities (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users (id) on delete cascade,
    provider text not null,
    provider_user_id text,
    password_hash text,
    created_at timestamptz not null default now(),
    
    unique (provider, provider_user_id),
    unique (user_id, provider)
);

create index user_identities_user_id_idx on user_identities (user_id);
