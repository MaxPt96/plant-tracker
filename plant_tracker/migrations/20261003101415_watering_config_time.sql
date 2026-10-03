-- Add migration script here

ALTER Table watering_config add COLUMN update_time TIMESTAMPTZ DEFAULT now();