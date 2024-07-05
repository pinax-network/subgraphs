use substreams::errors::Error;
use substreams::pb::substreams::store_delta::Operation;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams::store::{DeltaInt64, Deltas};

#[substreams::handlers::map]
fn block_index(min_block: Deltas<DeltaInt64>) -> Result<Keys, Error> {
    Ok(match min_block.deltas[0].operation != Operation::Create {
        true => Keys::default(),
        false => Keys {
            keys: vec!["clock".to_string()],
        },
    })
}
