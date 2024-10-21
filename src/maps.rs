use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams::store::{DeltaString, Deltas};

#[substreams::handlers::map]
pub fn clock(clock: Clock) -> Result<Clock, Error> {
    Ok(clock)
}

#[substreams::handlers::map]
pub fn map_clock(params: String, clock: Clock, store: Deltas<DeltaString>) -> Result<Clock, Error> {
    if params.is_empty() {
        return Ok(clock);
    }
    // only emit clock based on time interval
    for delta in store.deltas {
        if delta.key != params {
            continue;
        }
        if delta.old_value == "" {
            continue; // skips genesis block
        }
        if delta.old_value != delta.new_value {
            return Ok(clock);
        }
    }
    Ok(Clock::default()) // empty clock, block will be skipped
}
