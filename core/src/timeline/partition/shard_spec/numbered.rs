use super::ShardSpecImpl;
use crate::data::input_row::InputRow;
use crate::timeline::ShardSpec;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct NumberedShardSpec {}

impl ShardSpecImpl for NumberedShardSpec {
    fn is_in_chunk(&self, timestamp: u64, input_row: &impl InputRow) -> bool {
        unimplemented!()
    }

    fn get_partition_num(&self) -> u32 {
        unimplemented!()
    }

    fn get_domain_dimensions(&self) -> Vec<&str> {
        unimplemented!()
    }

    fn possible_in_domain(&self, domain: HashMap<String, HashSet<Range<&str>>>) -> bool {
        unimplemented!()
    }

    fn is_compatible(&self, other: &ShardSpec) -> bool {
        unimplemented!()
    }
}
