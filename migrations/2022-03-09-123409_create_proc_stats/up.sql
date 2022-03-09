-- Your SQL goes here
CREATE TABLE proc_stats (
   time                 TIMESTAMP       PRIMARY KEY NOT NULL,

   start_time           TIMESTAMP       NULL,
   exe                  TEXT            NULL,
   cmd                  TEXT            NULL,
   name                 TEXT            NULL,
   disk_read            BIGINT          NULL,
   disk_read_total      BIGINT          NULL,
   disk_written         BIGINT          NULL,
   disk_written_total   BIGINT          NULL,
   cpu_usage            REAL            NULL,
   rss                  BIGINT          NULL,
   status               TEXT            NULL
);

SELECT create_hypertable('proc_stats', 'time');
