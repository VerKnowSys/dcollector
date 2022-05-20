ALTER TABLE disk_stats ADD COLUMN host_name TEXT;
ALTER TABLE proc_stats ADD COLUMN host_name TEXT;

ALTER TABLE disk_stats ALTER COLUMN temperature TYPE DOUBLE PRECISION;
