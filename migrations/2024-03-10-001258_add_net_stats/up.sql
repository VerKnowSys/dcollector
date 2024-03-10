CREATE TABLE net_stats (
   time                         TIMESTAMP       PRIMARY KEY NOT NULL,

   netdev                       TEXT            NOT NULL,

   packets_received             BIGINT          NOT NULL,
   total_packets_received       BIGINT          NOT NULL,

   packets_transmitted          BIGINT          NOT NULL,
   total_packets_transmitted    BIGINT          NOT NULL,

   received                     BIGINT          NOT NULL,
   total_received               BIGINT          NOT NULL,

   transmitted                  BIGINT          NOT NULL,
   total_transmitted            BIGINT          NOT NULL,

   transmitted_errors           BIGINT          NOT NULL,
   transmitted_total_errors     BIGINT          NOT NULL,

   received_errors              BIGINT          NOT NULL,
   received_total_errors        BIGINT          NOT NULL

);

SELECT create_hypertable('net_stats', 'time');
