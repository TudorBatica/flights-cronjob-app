CREATE OR REPLACE FUNCTION over_weekend(
    arrive_time timestamptz, return_time timestamptz
) RETURNS boolean AS $$
BEGIN
    RETURN (return_time::date - arrive_time::date) > 6 OR extract(dow from return_time) < extract(dow from arrive_time);
END;
$$ LANGUAGE plpgsql;