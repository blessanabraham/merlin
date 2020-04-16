use super::partition::ShardSpec;
use std::time::Duration;

/// Identifier of {@link DataSegment}.
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct SegmentId {
    datasource: String,
    interval_start_millis: u64,
    interval_end_millis: u64,
    version: String,
    partition_num: u32,
}

impl SegmentId {
    // Implementation note: this class must be optimized for resident memory footprint, because segment data consumes
    // a lot of heap memory on Merlin Broker and Coordinator nodes.
    //
    // This class is separate from SegmentIdWithShardSpec
    // because in a lot of places segment ids are transmitted as "segment id strings" that don't contain enough
    // information to deconstruct the ShardSpec. Also, even a single extra field is important for SegmentIds, because it
    // adds to the memory footprint considerably.
    //
    // The difference between this class and {@link SegmentDescriptor} is that the latter is
    // a "light" version of SegmentId, that only contains the interval, version, and partition number. It's used where the
    // data source, another essential part of SegmentId is determined by the context (e. g. in {@link CachingClusteredClient},
    // where SegmentDescriptor is used when Brokers tell data servers
    // which segments to include for a particular query) and where having lean JSON representations is important, because
    // it's actively transferred between Merlin nodes. It's also for this reason that the JSON field names of
    // SegmentDescriptor are abbreviated.
    //
    // API design note: "SegmentId" is chosen as the name for this class instead of more verbose "SegmentIdentifier" or
    // "DataSegmentIdentifier" because it's used very frequently and a long class name adds noticeable clutter. Variables
    // of SegmentId type are recommended to be named "segmentId" rather than "identifier" or "segmentIdentifier".

    const DELIMITER: char = '_';

    // pub fn of<T: ShardSpec>(
    //     datasource: &str,
    //     interval: Duration,
    //     version: &str,
    //     shard_spec: Option<&T>,
    // ) -> Self {
    // }
}
