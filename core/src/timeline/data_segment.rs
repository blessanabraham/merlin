use super::segment_id::SegmentId;

/// Metadata of Merlin's data segment. An immutable object.
///
/// DataSegment's equality ({@link #equals}/{@link #hashCode}) and {@link #compareTo} methods consider only the
/// {@link SegmentId} of the segment.
#[derive(Debug)]
pub struct DataSegment {
    binary_version: u32,
    id: SegmentId,
}
