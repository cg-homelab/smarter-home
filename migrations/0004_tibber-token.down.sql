-- Remove Tibber token column from homes table
ALTER TABLE homes
  DROP COLUMN tibber_token;

