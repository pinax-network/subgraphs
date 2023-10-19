CREATE TABLE IF NOT EXISTS Clock (
    seconds UInt32,
    nanos UInt16,
)
ENGINE = MergeTree()
ORDER BY (seconds)
