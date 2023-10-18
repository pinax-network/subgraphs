CREATE TABLE IF NOT EXISTS Clock (
    id FixedString(40),
    number String,
    seconds String,
    nanos String,
    timestamp String
)
ENGINE = ReplacingMergeTree()
ORDER BY (id)
