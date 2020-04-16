use super::row::Metric;
use super::row::{Row, RowImpl};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct MapBasedRow {}

impl RowImpl for MapBasedRow {
    fn get_timestamp_from_epoch(&self) -> u64 {
        unimplemented!()
    }

    fn get_timestamp(&self) -> DateTime<Utc> {
        unimplemented!()
    }

    fn get_dimension(&self, dimension: &str) -> Vec<String> {
        unimplemented!()
    }

    fn get_raw(&self, dimension: &str) -> Option<&Row> {
        unimplemented!()
    }

    fn get_metric(&self, metric: &str) -> Option<Metric> {
        unimplemented!()
    }
}
