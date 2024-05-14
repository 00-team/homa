
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
    amount integer not null,
    vendor_order_id text,
    vendor_track_id integer,
    card_number text,
    hashed_card_number text,
    date integer,
    bank_track_id integer
);

create table if not exists general (
    available_money integer not null default 0,
    total_money integer not null default 0,
    rub_irr integer not null default 0,
    rub_irr_update integer not null default 0
);

create table if not exists orders (
    id integer primary key not null,
    user integer not null references users(id) on delete cascade,
    status integer not null default 0,
    activation_id integer not null,
    phone text not null,
    cost integer not null,
    cc integer not null,
    operator text not null,
    datetime text not null
);
create index if not exists orders_activation_id on orders(activation_id);
create index if not exists orders_user on orders(user);

create table if not exists prices (
    id integer primary key not null,
    country integer not null,
    service text not null,
    purchase_cost integer not null,
    current_cost integer not null
);
