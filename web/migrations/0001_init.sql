
create table if not exists users (
    id integer primary key not null,
    name text not null,
    auth_date integer not null,
    username text,
    wallet integer not null default 0,
    in_hold integer not null default 0,
    token text not null,
    photo boolean not null default false,
    admin boolean not null default false,
    banned boolean not null default false
);

create table if not exists transactions (
    id integer primary key not null,
    user integer not null references users(id) on delete cascade,
    kind integer not null default 0, -- in OR out | withdrawl OR deposit
    status integer not null default 0, -- success | failed | in progress
    timestamp integer not null,
    amount integer not null,
    authority text not null,
    ref_id integer,
    card text,
    card_hash text
);

create table if not exists general (
    money_total integer not null default 0,
    money_gain integer not null default 0,
    money_loss integer not null default 0,
    rub_irr integer not null default 0,
    rub_irr_update integer not null default 0,
    usd_irr integer not null default 0,
    usd_irr_update integer not null default 0,
    star_tax integer not null default 0,
    phone_tax integer not null default 0,
    price_diff_total integer not null default 0,
    price_diff_count integer not null default 0,
    prices text not null default '{}',
    prices_update integer not null default 0,
    disable_wallet boolean not null default false,
    disable_stars boolean not null default false,
    disable_phone boolean not null default false
);

create table if not exists phone_orders (
    id integer primary key not null,
    user integer not null references users(id) on delete cascade,
    status integer not null default 0,
    activation_id integer not null,
    phone text not null,
    cost integer not null,
    operator text not null,
    datetime text not null,
    country text not null,
    service text not null
);
create index if not exists phone_orders_aid on phone_orders(activation_id);
create index if not exists phone_orders_user on phone_orders(user);

create table if not exists star_orders (
    id integer primary key not null,
    user integer not null references users(id) on delete cascade,
    status integer not null default 0,
    amount integer not null,
    cost integer not null,
    timestamp integer not null
);
create index if not exists star_orders_user on star_orders(user);

create table if not exists messages (
    id integer primary key not null,
    user integer not null references users(id) on delete cascade,
    activation_id integer not null,
    timestamp integer not null,
    text text not null,
    code text not null,
    country text not null,
    service text not null,
    received_at text not null,
    seen boolean not null default false
);

