ALTER TABLE homes
    DROP COLUMN IF EXISTS location_hash_low,
    DROP COLUMN IF EXISTS location_hash_medium,
    DROP COLUMN IF EXISTS location_hash_high,
    DROP COLUMN IF EXISTS longitude,
    DROP COLUMN IF EXISTS latitude;
