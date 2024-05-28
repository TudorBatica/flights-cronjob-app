CREATE TYPE location_type_enum as ENUM (
    'continent',
    'region',
    'country',
    'subdivision',
    'autonomous',
    'city',
    'airport'
);

CREATE TABLE locations
(
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    continent_id VARCHAR(255),
    region_id VARCHAR(255),
    country_id VARCHAR(255),
    subdivision_id VARCHAR(255),
    autonomous_id VARCHAR(255),
    city_id VARCHAR(255),
    location_type location_type_enum NOT NULL,
    CONSTRAINT fk_continent FOREIGN KEY (continent_id) REFERENCES locations(id) ON DELETE SET NULL,
    CONSTRAINT fk_region FOREIGN KEY (region_id) REFERENCES locations(id) ON DELETE SET NULL,
    CONSTRAINT fk_country FOREIGN KEY (country_id) REFERENCES locations(id) ON DELETE SET NULL,
    CONSTRAINT fk_subdivision FOREIGN KEY (subdivision_id) REFERENCES locations(id) ON DELETE SET NULL,
    CONSTRAINT fk_autonomous FOREIGN KEY (autonomous_id) REFERENCES locations(id) ON DELETE SET NULL,
    CONSTRAINT fk_city FOREIGN KEY (city_id) REFERENCES locations(id) ON DELETE SET NULL
);
CREATE INDEX idx_locations_name ON locations(name);

CREATE TABLE routes
(
    from_location_id  VARCHAR(255) NOT NULL,
    to_location_id VARCHAR(255) NOT NULL,
    last_scan    TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (from_location_id, to_location_id),
    CONSTRAINT fk_from_location FOREIGN KEY (from_location_id) REFERENCES locations(id) ON DELETE CASCADE,
    CONSTRAINT fk_to_location FOREIGN KEY (to_location_id) REFERENCES locations(id) ON DELETE CASCADE
);
CREATE INDEX idx_routes_last_scan ON routes(last_scan);

CREATE TABLE user_routes
(
    user_id INTEGER NOT NULL,
    from_location_id VARCHAR(255) NOT NULL,
    to_location_id VARCHAR(255) NOT NULL,
    PRIMARY KEY (user_id, from_location_id, to_location_id),
    CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users(id),
    CONSTRAINT fk_route FOREIGN KEY (from_location_id, to_location_id) REFERENCES routes(from_location_id, to_location_id) ON DELETE CASCADE
);
CREATE INDEX idx_user_routes_user_id ON user_routes(user_id);

CREATE TYPE itinerary_type_enum as ENUM (
    'weekend',
    'weekly',
    'long'
);

CREATE TABLE itineraries
(
    id SERIAL PRIMARY KEY,
    from_airport_id VARCHAR(255) NOT NULL,
    to_airport_id VARCHAR(255) NOT NULL,
    price SMALLINT NOT NULL,
    itinerary_type itinerary_type_enum NOT NULL,
    booking_link TEXT NOT NULL,
    departure_depart_at_utc TIMESTAMPTZ NOT NULL,
    departure_arrive_at_utc TIMESTAMPTZ NOT NULL,
    return_depart_at_utc TIMESTAMPTZ NOT NULL,
    return_arrive_at_utc TIMESTAMPTZ NOT NULL,
    stopovers SMALLINT NOT NULL,
    inserted_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT fk_from_airport_id FOREIGN KEY (from_airport_id) REFERENCES locations(id),
    CONSTRAINT fk_to_airport_id FOREIGN KEY (to_airport_id) REFERENCES locations(id)
);

CREATE TABLE flights
(
    id SERIAL PRIMARY KEY,
    itinerary_id INTEGER,
    from_airport_id VARCHAR(255) NOT NULL,
    to_airport_id VARCHAR(255) NOT NULL,
    depart_at_utc TIMESTAMPTZ NOT NULL,
    arrive_at_utc TIMESTAMPTZ NOT NULL,
    airline VARCHAR(20),
    flight_number INTEGER
);


