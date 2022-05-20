CREATE TABLE disk_stats (
   time                 TIMESTAMP       PRIMARY KEY NOT NULL,

   name                 TEXT            NULL,
   temperature          REAL            NULL,
   crc_errors           BIGINT          NULL,
   seek_time            BIGINT          NULL,
   seek_error_rate      BIGINT          NULL,
   throughput           BIGINT          NULL,
   read_error_rate      BIGINT          NULL
);

SELECT create_hypertable('disk_stats', 'time');
