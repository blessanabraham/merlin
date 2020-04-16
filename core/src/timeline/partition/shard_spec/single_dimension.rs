use super::ShardSpecImpl;
use crate::data::input_row::InputRow;
use crate::timeline::ShardSpec;
use std::collections::{HashMap, HashSet};
use std::ops::{Range, RangeFrom};

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SingleDimensionShardSpec {
    partition_num: u32,
    dimension: String,
    start: Option<String>,
    end: Option<String>,
}

impl SingleDimensionShardSpec {
    pub fn get_dimension(&self) -> &str {
        &self.dimension
    }

    pub fn get_start(&self) -> Option<&String> {
        self.start.as_ref()
    }

    pub fn get_end(&self) -> Option<&String> {
        self.end.as_ref()
    }

    fn get_range(&self) -> Range<String> {
        unimplemented!()
    }
}

impl ShardSpecImpl for SingleDimensionShardSpec {
    fn is_in_chunk(&self, timestamp: u64, input_row: &impl InputRow) -> bool {
        unimplemented!()
    }

    fn get_partition_num(&self) -> u32 {
        self.partition_num
    }

    fn get_domain_dimensions(&self) -> Vec<&str> {
        vec![&self.dimension]
    }

    fn possible_in_domain(&self, domain: HashMap<String, HashSet<Range<&str>>>) -> bool {
        unimplemented!()
    }

    fn is_compatible(&self, other: &ShardSpec) -> bool {
        unimplemented!()
    }
}
