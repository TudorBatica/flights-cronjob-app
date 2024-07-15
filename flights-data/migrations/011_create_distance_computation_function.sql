CREATE OR REPLACE FUNCTION distance_km(
    lat1 float, lon1 float,
    lat2 float, lon2 float
) RETURNS integer AS $$
DECLARE
    earth_radius_km float := 6371;
    dlat float;
    dlon float;
    a float;
    c float;
    distance int;
BEGIN
    lat1 := radians(lat1);
    lon1 := radians(lon1);
    lat2 := radians(lat2);
    lon2 := radians(lon2);

    dlat := lat2 - lat1;
    dlon := lon2 - lon1;

    a := sin(dlat/2)^2 + cos(lat1) * cos(lat2) * sin(dlon/2)^2;
    c := 2 * asin(sqrt(a));

    RETURN FLOOR(earth_radius_km * c);
END;
$$ LANGUAGE plpgsql;