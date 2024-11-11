USE restaurant_data;

CREATE TABLE IF NOT EXISTS businesses (
    id UUID PRIMARY KEY,
    yelp_id TEXT,
    biz_name TEXT,
    supports_pickup BOOLEAN,
    supports_delivery BOOLEAN,
    yelp_rating FLOAT,
    latitude FLOAT,
    longitude FLOAT,
    price_magnitude INT,
    phone_number TEXT,
    website_url TEXT,
    reservation_link TEXT,
    building_number INT,
    street TEXT,
    city TEXT,
    state TEXT
);

CREATE TABLE IF NOT EXISTS menu_data (
    item_id TIMEUUID,
    business_id UUID,
    item_name TEXT,
    item_type TEXT, -- STARTER, MAIN, DESSERT, DRINK, BOTTLE, SIDE
    item_desc TEXT,
    item_price FLOAT,
    PRIMARY KEY (item_id, business_id)
);

CREATE TABLE IF NOT EXISTS text_data (
    entry_id TIMEUUID PRIMARY KEY,
    business_id UUID,
    text_selection TEXT,
    source TEXT,
    embedding vector<float, 4096>
);


CREATE CUSTOM INDEX IF NOT EXISTS text_data_vector_idx 
ON text_data(embedding) 
USING 'StorageAttachedIndex' 
WITH OPTIONS = {
    'similarity_function': 'cosine'
};

CREATE TABLE IF NOT EXISTS opening_data (
	entry_id TIMEUUID,
	business_id UUID,
	/* Both in 24hour syntax (ie 0124, 1245, 1523, etc) */
	open_time int,
	close_time int,
	day_of_week int, --Assuming week starts on Monday (fuck you yelp),
	PRIMARY KEY(entry_id, business_id)
);

CREATE TABLE IF NOT EXISTS locations (
	location_id UUID PRIMARY KEY,
	location_name text,
	boundaries list<frozen<tuple<double, double>>>
);

CREATE TABLE IF NOT EXISTS location_data (
	entry_id TIMEUUID,
	location_id UUID,
	data text,
	embedding vector<float, 4096>,
	PRIMARY KEY(entry_id, location_id)
);


CREATE CUSTOM INDEX IF NOT EXISTS location_data_vector_idx
ON location_data(embedding) 
USING 'StorageAttachedIndex' 
WITH OPTIONS = {
    'similarity_function': 'cosine'
};
