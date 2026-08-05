ALTER TABLE homes
    ADD COLUMN IF NOT EXISTS latitude DOUBLE PRECISION DEFAULT 59.9139,
    ADD COLUMN IF NOT EXISTS longitude DOUBLE PRECISION DEFAULT 10.7522,
    ADD COLUMN IF NOT EXISTS location_hash_high TEXT DEFAULT '860999387ffffff',
    ADD COLUMN IF NOT EXISTS location_hash_medium TEXT DEFAULT '830999fffffffff',
    ADD COLUMN IF NOT EXISTS location_hash_low TEXT DEFAULT '8109bffffffffff';

UPDATE homes
SET
    latitude = COALESCE(latitude, 59.9139),
    longitude = COALESCE(longitude, 10.7522),
    location_hash_high = COALESCE(location_hash_high, '860999387ffffff'),
    location_hash_medium = COALESCE(location_hash_medium, '830999fffffffff'),
    location_hash_low = COALESCE(location_hash_low, '8109bffffffffff');

ALTER TABLE homes
    ALTER COLUMN latitude SET NOT NULL,
    ALTER COLUMN longitude SET NOT NULL,
    ALTER COLUMN location_hash_high SET NOT NULL,
    ALTER COLUMN location_hash_medium SET NOT NULL,
    ALTER COLUMN location_hash_low SET NOT NULL;

ALTER TABLE homes
    ALTER COLUMN latitude DROP DEFAULT,
    ALTER COLUMN longitude DROP DEFAULT,
    ALTER COLUMN location_hash_high DROP DEFAULT,
    ALTER COLUMN location_hash_medium DROP DEFAULT,
    ALTER COLUMN location_hash_low DROP DEFAULT;