use substreams::pb::substreams::Clock;
use substreams::store::{StoreNew, StoreSet, StoreSetString};

#[substreams::handlers::store]
pub fn store_clock(clock: Clock, store: StoreSetString) {
    if clock.number == 0 {
        return;
    }
    let timestamp = clock.timestamp.as_ref().expect("missing timestamp");
    let block_date = to_date(&clock);
    let minute = timestamp.seconds / 60;
    let minute_3 = timestamp.seconds / 60 * 3;
    let minute_5 = timestamp.seconds / 60 * 5;
    let minute_30 = timestamp.seconds / 60 * 30;
    let hour = timestamp.seconds / 3600;
    let hour_2 = timestamp.seconds / 3600 * 2;
    let hour_6 = timestamp.seconds / 3600 * 6;
    let hour_8 = timestamp.seconds / 3600 * 8;
    let hour_12 = timestamp.seconds / 3600 * 12;
    let day = timestamp.seconds / 86400;
    let day_3 = timestamp.seconds / 86400 * 3;

    store.set(0, "1m", &minute.to_string());
    store.set(0, "3m", &minute_3.to_string());
    store.set(0, "5m", &minute_5.to_string());
    store.set(0, "30m", &minute_30.to_string());
    store.set(0, "1h", &hour.to_string());
    store.set(0, "2h", &hour_2.to_string());
    store.set(0, "6h", &hour_6.to_string());
    store.set(0, "8h", &hour_8.to_string());
    store.set(0, "12h", &hour_12.to_string());
    store.set(0, "1d", &day.to_string());
    store.set(0, "3d", &day_3.to_string());
    store.set(0, "m", &to_month(&block_date));
    store.set(0, "y", &to_year(&block_date));
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
