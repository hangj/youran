-- Add migration script here
-- The datatype is optional. 
-- see <https://www.sqlite.org/quirks.html#the_datatype_is_optional>
-- and <https://www.sqlite.org/datatype3.html#datatypes_in_sqlite>
CREATE TABLE IF NOT EXISTS tb_kv(
    key primary key,
    value,
    timestamp DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_timestamp ON tb_kv(timestamp);
