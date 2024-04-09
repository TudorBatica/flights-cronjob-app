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

CREATE TABLE monitored_routes
(
    monitored_by INTEGER,
    airport_code TEXT,
    country_code TEXT,
    budget       SMALLINT,
    trip_type    SMALLINT,
    FOREIGN KEY (monitored_by) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (airport_code) REFERENCES airports (code) ON DELETE CASCADE,
    FOREIGN KEY (country_code) REFERENCES countries (code) ON DELETE CASCADE,
    PRIMARY KEY (monitored_by, airport_code, country_code)
);

CREATE TABLE trips
(
    trip_id      SERIAL PRIMARY KEY,
    airport_code TEXT,
    country_code TEXT,
    depart_at    DATE,
    return_at    DATE,
    price        SMALLINT,
    trip_type    SMALLINT,
    inserted_at  DATE,
    city_code    TEXT,
    city_name    TEXT,
    FOREIGN KEY (airport_code) REFERENCES airports (code) ON DELETE CASCADE,
    FOREIGN KEY (country_code) REFERENCES countries (code) ON DELETE CASCADE
);

