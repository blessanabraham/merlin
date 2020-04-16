mod single_dimension;
pub use single_dimension::SingleDimensionShardSpec;

mod linear;
pub use linear::LinearShardSpec;

mod numbered;
pub use numbered::NumberedShardSpec;

mod hash_based_numbered;
pub use hash_based_numbered::HashBasedNumberedShardSpec;

mod numbered_overwrite;
pub use numbered_overwrite::NumberedOverwriteShardSpec;

use crate::data::input_row::InputRow;
use enum_dispatch::enum_dispatch;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

#[enum_dispatch]
#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ShardSpec {
    Single(SingleDimensionShardSpec),
    Linear(LinearShardSpec),
    Numbered(NumberedShardSpec),
    Hashed(HashBasedNumberedShardSpec),
    NumberedOverwrite(NumberedOverwriteShardSpec),
}

/// A Marker trait that exists to combine ShardSpec objects together
#[enum_dispatch(ShardSpec)]
trait ShardSpecImpl {
    fn is_in_chunk(&self, timestamp: u64, input_row: &impl InputRow) -> bool;

    fn get_partition_num(&self) -> u32;

    fn get_start_root_partition_id(&self) -> u32 {
        self.get_partition_num()
    }

    fn get_end_root_partition_id(&self) -> u32 {
        self.get_partition_num() + 1
    }

    fn get_minor_version(&self) -> u16 {
        0
    }

    fn get_atomic_update_group_size(&self) -> u16 {
        1
    }

    /// Get dimensions who have possible range for the rows this shard contains.
    fn get_domain_dimensions(&self) -> Vec<&str>;

    /// if given domain ranges are not possible in this shard, return false; otherwise return true;
    fn possible_in_domain(&self, domain: HashMap<String, HashSet<Range<&str>>>) -> bool;

    /// Returns true if two segments of this and other shardSpecs can exist in the same timeChunk.
    fn is_compatible(&self, other: &ShardSpec) -> bool;
}
