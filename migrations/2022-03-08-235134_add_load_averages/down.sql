-- This file should undo anything in `up.sql`
ALTER TABLE sys_stats DROP IF EXISTS load_one;
ALTER TABLE sys_stats DROP IF EXISTS load_five;
ALTER TABLE sys_stats DROP IF EXISTS load_fifteen;
