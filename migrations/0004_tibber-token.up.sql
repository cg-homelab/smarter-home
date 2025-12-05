-- Create Tibber token column in homes table
ALTER TABLE homes
  ADD COLUMN tibber_token VARCHAR(255);
