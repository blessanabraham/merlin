use super::MapBasedRow;
use chrono::{DateTime, Utc};
use enum_dispatch::enum_dispatch;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Metric {
    I8(i8),
    I16(i16),
    I32(i32),
    I164(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
}

#[enum_dispatch]
#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(tag = "version", rename_all = "lowercase")]
pub enum Row {
    V1(MapBasedRow),
}

/// A Row of data.  This can be used for both input and output into various parts of the system.  It assumes
/// that the user already knows the schema of the row and can query for the parts that they care about.
#[enum_dispatch(Row)]
pub(crate) trait RowImpl {
    /// Returns the timestamp from the epoch in milliseconds.  If the event happened _right now_, this would return the
    /// same thing as System.currentTimeMillis();
    fn get_timestamp_from_epoch(&self) -> u64;

    /// Returns the timestamp from the epoch as an org.joda.time.DateTime.  If the event happened _right now_, this would return the
    /// same thing as new DateTime();
    fn get_timestamp(&self) -> DateTime<Utc>;

    /// Returns the list of dimension values for the given column name.
    fn get_dimension(&self, dimension: &str) -> Vec<String>;

    /// Returns the raw dimension value for the given column name. This is different from {@link #getDimension} which
    /// converts all values to strings before returning them.
    fn get_raw(&self, dimension: &str) -> Option<&Row>;

    /// Returns the metric column value for the given column name. This method is different from {@link #getRaw} in two
    /// aspects:
    ///  1. If the column is absent in the row, either numeric zero or null will be returned, depending on
    ///     the value of merlin.generic.useDefaultValueForNull.
    ///  2. If the column has string value, an attempt is made to parse this value as a number.
    fn get_metric(&self, metric: &str) -> Option<Metric>;
}
