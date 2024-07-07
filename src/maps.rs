use substreams::errors::Error;
use substreams::pb::substreams::Clock;

#[substreams::handlers::map]
pub fn map_clock(clock: Clock) -> Result<Clock, Error> {
    Ok(clock)
}