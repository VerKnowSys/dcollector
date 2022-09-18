-- Your SQL goes here

ALTER TABLE sys_stats ALTER COLUMN total_memory TYPE BIGINT;
ALTER TABLE sys_stats ALTER COLUMN used_memory TYPE BIGINT;
ALTER TABLE sys_stats ALTER COLUMN total_swap TYPE BIGINT;
ALTER TABLE sys_stats ALTER COLUMN used_swap TYPE BIGINT;
