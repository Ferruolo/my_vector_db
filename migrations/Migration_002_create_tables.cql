CREATE TABLE IF NOT EXISTS businesses (
	id UUID PRIMARY KEY,
	yelp_id text,
	biz_name text,
	supports_pickup boolean,
	supports_delivery boolean,
	yelp_rating float,
	latitude float,
	longitude float,
	price_magnitude int,
	phone_number text,
	website_url text,
	reservation_link text null
	building_number int,
	street text,
	city text,
	state text,
);

CREATE TABLE IF NOT EXISTS menu_data (
	item_id TIMEUUID,
	business_id UUID,
	item_name text,
	item_type text, --::STARTER, MAIN, DESSERT, DRINK, BOTTLE, SIDE
	item_desc text NULL,
	item_price float NULL,
	PRIMARY KEY (item_id, business_id)
);

CREATE TABLE IF NOT EXISTS text_data (
	entry_id TIMEUUID PRIMARY KEY,
	business_id UUID,
	text_selection text,
	source text,
	embedding vector<float, 4096>,
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
	day_of_week int, --Assuming week starts on Monday (fuck you yelp)
);

CREATE TABLE IF NOT EXISTS locations (
	location_id UUID,
	location_name text,
	boundaries list<frozen<tuple<double, double>>>
);

CREATE TABLE IF NOT EXISTS location_data (
	location_id UUID,
	data text,
	embedding vector<float, 4096>,
);


CREATE CUSTOM INDEX IF NOT EXISTS location_data_vector_idx
ON text_data(embedding) 
USING 'StorageAttachedIndex' 
WITH OPTIONS = {
    'similarity_function': 'cosine'
};
