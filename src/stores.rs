use substreams::pb::substreams::Clock;
use substreams::store::{StoreMin, StoreMinInt64, StoreNew};

#[substreams::handlers::store]
pub fn store_min_block(clock: Clock, store: StoreMinInt64) {
    let day = clock.timestamp.unwrap().seconds / 86400;
    store.min(0, day.to_string(), clock.number as i64);
}