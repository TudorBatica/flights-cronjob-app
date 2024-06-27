ALTER TABLE flights 
ADD CONSTRAINT fk_route FOREIGN KEY (itinerary_id) REFERENCES itineraries(id) ON DELETE CASCADE;