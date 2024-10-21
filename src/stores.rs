use substreams::pb::substreams::Clock;
use substreams::store::{StoreNew, StoreSet, StoreSetString};

#[substreams::handlers::store]
pub fn store_clock(clock: Clock, store: StoreSetString) {
    if clock.number == 0 {
        return;
    }
    let timestamp = clock.timestamp.as_ref().expect("missing timestamp");
    let block_date = to_date(&clock);
    let hour = timestamp.seconds / 3600;
    store.set(0, "hour", &hour.to_string());
    store.set(0, "day", &block_date);
    store.set(0, "month", &to_month(&block_date));
    store.set(0, "year", &to_year(&block_date));
}

// Clock to date string
// ex: Clock => 2015-07-30
pub fn to_date(clock: &Clock) -> String {
    let timestamp = clock.timestamp.as_ref().expect("missing timestamp");
    timestamp
        .to_string()
        .split('T')
        .next()
        .expect("missing date")
        .to_string()
}

// Timestamp to date conversion
// ex: 2015-07-30 => 2015-07
pub fn to_month(block_date: &str) -> String {
    match block_date
        .split('-')
        .take(2)
        .collect::<Vec<&str>>()
        .join("-")
        .as_str()
    {
        date => date.to_string(),
    }
}

pub fn to_year(block_date: &str) -> String {
    match block_date.split('-').next() {
        Some(date) => date.to_string(),
        None => "".to_string(),
    }
}
