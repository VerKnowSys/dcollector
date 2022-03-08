-- Your SQL goes here
CREATE TABLE sys_stats (
   time             TIMESTAMP         PRIMARY KEY NOT NULL,

   name             TEXT              NULL,
   kernel_version   TEXT              NULL,

   os_version       TEXT              NULL,
   host_name        TEXT              NULL,
   processors       INTEGER           NULL,

   total_memory     INTEGER           NULL,
   used_memory      INTEGER           NULL,
   total_swap       INTEGER           NULL,
   used_swap        INTEGER           NULL
);

SELECT create_hypertable('sys_stats', 'time');
