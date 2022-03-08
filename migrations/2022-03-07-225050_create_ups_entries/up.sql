-- Your SQL goes here
CREATE TABLE ups_stats (
   time             TIMESTAMP         PRIMARY KEY NOT NULL,
   model            TEXT              NULL,
   status           TEXT              NULL,
   load             INTEGER           NULL,

   input_frequency  DOUBLE PRECISION  NULL,
   input_voltage    DOUBLE PRECISION  NULL,

   battery_charge   INTEGER           NULL,
   battery_voltage  DOUBLE PRECISION  NULL
);

SELECT create_hypertable('ups_stats', 'time');
