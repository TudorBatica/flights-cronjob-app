CREATE OR REPLACE FUNCTION notify_route_inserted()
RETURNS TRIGGER AS $$
BEGIN
    PERFORM pg_notify('route_inserted_channel', row_to_json(NEW)::text);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER route_inserted_trigger
AFTER INSERT ON routes
FOR EACH ROW
EXECUTE FUNCTION notify_route_inserted();