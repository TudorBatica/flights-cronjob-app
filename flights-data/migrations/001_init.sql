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
    budget       SMALLINT NOT NULL,
    trip_type    SMALLINT NOT NULL,
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
    depart_at    TIMESTAMPTZ NOT NULL,
    return_at    TIMESTAMPTZ NOT NULL,
    price        SMALLINT   NOT NULL,
    trip_type    SMALLINT   NOT NULL,
    inserted_at  TIMESTAMPTZ NOT NULL,
    city_code    TEXT       NOT NULL,
    city_name    TEXT       NOT NULL,
    FOREIGN KEY (airport_code) REFERENCES airports (code) ON DELETE CASCADE,
    FOREIGN KEY (country_code) REFERENCES countries (code) ON DELETE CASCADE
);

