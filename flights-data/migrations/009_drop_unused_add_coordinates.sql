ALTER TABLE itineraries
DROP COLUMN IF EXISTS itinerary_type;

DROP TYPE IF EXISTS itinerary_type_enum;

ALTER TABLE monitored_trips
DROP COLUMN IF EXISTS departure_id;

ALTER TABLE locations
ADD COLUMN longitude DOUBLE PRECISION,
ADD COLUMN latitude DOUBLE PRECISION;