CREATE TABLE users
(
    id    SERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    name  TEXT NOT NULL
);

CREATE TABLE airports
(
    code TEXT PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE countries
(
    code TEXT PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE routes
(
    PRIMARY KEY (airport_code, country_code),
    airport_code TEXT,
    country_code TEXT,
    FOREIGN KEY (airport_code) REFERENCES airports (code) ON DELETE CASCADE,
    FOREIGN KEY (country_code) REFERENCES countries (code) ON DELETE CASCADE
);

CREATE TABLE user_routes
(
    user_id      INTEGER,
    airport_code TEXT,
    country_code TEXT,
    budget       SMALLINT,
    trip_type    SMALLINT,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (airport_code, country_code) REFERENCES routes (airport_code, country_code) ON DELETE CASCADE,
    PRIMARY KEY (user_id, airport_code, country_code)
);

CREATE TABLE trips
(
    trip_id      SERIAL PRIMARY KEY,
    airport_code TEXT,
    country_code TEXT,
    depart_at    DATE,
    arrive_at    DATE,
    price        SMALLINT,
    airline      TEXT,
    trip_type    SMALLINT,
    inserted_at  DATE,
    city_code    TEXT,
    city_name    TEXT,
    FOREIGN KEY (airport_code, country_code) REFERENCES routes (airport_code, country_code) ON DELETE CASCADE
);

