-- This file should undo anything in `up.sql`
ALTER TABLE sys_stats ALTER COLUMN total_memory TYPE INTEGER;
ALTER TABLE sys_stats ALTER COLUMN used_memory TYPE INTEGER;
ALTER TABLE sys_stats ALTER COLUMN total_swap TYPE INTEGER;
ALTER TABLE sys_stats ALTER COLUMN used_swap TYPE INTEGER;
