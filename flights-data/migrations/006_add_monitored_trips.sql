CREATE TABLE monitored_trips
(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    departure_id VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_departure FOREIGN KEY (departure_id) REFERENCES locations(id) ON DELETE CASCADE
);
CREATE INDEX idx_monitored_trip_user_id ON monitored_trips(user_id);

CREATE TABLE trip_routes
(
    from_airport_id  VARCHAR(255) NOT NULL,
    to_airport_id VARCHAR(255) NOT NULL,
    monitored_trip_id INTEGER NOT NULL,
    PRIMARY KEY (from_airport_id, to_airport_id),
    CONSTRAINT fk_from_airport FOREIGN KEY (from_airport_id) REFERENCES locations(id) ON DELETE CASCADE,
    CONSTRAINT fk_to_airport FOREIGN KEY (to_airport_id) REFERENCES locations(id) ON DELETE CASCADE,
    CONSTRAINT fk_monitored_trip_id FOREIGN KEY (monitored_trip_id) REFERENCES monitored_trips(id) ON DELETE CASCADE
);
CREATE INDEX idx_routes_monitored_trip_id ON trip_routes(monitored_trip_id);