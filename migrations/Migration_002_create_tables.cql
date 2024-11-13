USE restaurant_data;

CREATE TABLE IF NOT EXISTS businesses
(
    id                UUID PRIMARY KEY,
    yelp_id           TEXT,
    biz_name          TEXT,
    supports_pickup   BOOLEAN,
    supports_delivery BOOLEAN,
    yelp_rating       FLOAT,
    phone_number      TEXT,
    website_url       TEXT,
);


CREATE TABLE IF NOT EXISTS biz_locations
(
    location_id     UUID,
    biz_id          UUID,
    latitude        FLOAT,
    longitude       FLOAT,
    building_number INT,
    room_number     TEXT,
    street          TEXT,
    city            TEXT,
    state           TEXT,
    PRIMARY KEY (location_id, biz_id)
);

CREATE TABLE IF NOT EXISTS menu_data
(
    item_id     UUID,
    business_id UUID,
    item_name   TEXT,
    item_type   TEXT, -- STARTER, MAIN, DESSERT, DRINK, BOTTLE, SIDE
    item_desc   TEXT,
    item_price  FLOAT,
    PRIMARY KEY (item_id, business_id)
);

CREATE TABLE IF NOT EXISTS text_data
(
    entry_id       UUID PRIMARY KEY,
    business_id    UUID,
    text_selection TEXT,
    embedding      vector<float, 4096>
);


CREATE CUSTOM INDEX IF NOT EXISTS text_data_vector_idx ON text_data (embedding) USING 'StorageAttachedIndex' WITH OPTIONS = { 'similarity_function': 'cosine' };

CREATE TABLE IF NOT EXISTS opening_data
(
    entry_id    UUID,
    business_id UUID,
    /* Both in 24hour syntax (ie 0124, 1245, 1523, etc) */
    open_time   int,
    close_time  int,
    day_of_week int, --Assuming week starts on Monday (fuck you yelp),
    PRIMARY KEY (entry_id, business_id)
);

CREATE TABLE IF NOT EXISTS locations
(
    location_id   UUID PRIMARY KEY,
    location_name text,
    boundaries    list<frozen<tuple<double, double>>>
);

CREATE TABLE IF NOT EXISTS location_data
(
    entry_id    UUID,
    location_id UUID,
    data        text,
    embedding   vector<float, 4096>,
    PRIMARY KEY (entry_id, location_id)
);


CREATE CUSTOM INDEX IF NOT EXISTS location_data_vector_idx ON location_data (embedding) USING 'StorageAttachedIndex' WITH OPTIONS = { 'similarity_function': 'cosine' };


-- Main reservations table with basic info
CREATE TABLE business_reservations
(
    business_id          UUID,
    accepts_reservations boolean,
    reservation_policy   text,
    PRIMARY KEY (business_id)
);

-- Reservation platforms table using composite key
CREATE TABLE business_reservation_platforms
(
    business_id   UUID,
    platform_type text, -- RESY, OPENTABLE, TOCK, YELP, DIRECT, OTHER
    url           text,
    notes         text,
    PRIMARY KEY (business_id, platform_type)
);

-- Reservation restrictions table using composite key
CREATE TABLE business_reservation_restrictions
(
    business_id         UUID,
    restriction_type    text,
    restriction_details text,
    PRIMARY KEY (business_id, restriction_type)
);
