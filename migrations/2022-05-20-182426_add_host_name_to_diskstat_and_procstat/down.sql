ALTER TABLE disk_stats DROP IF EXISTS host_name;
ALTER TABLE proc_stats DROP IF EXISTS host_name;

ALTER TABLE disk_stats ALTER COLUMN temperature TYPE REAL;

