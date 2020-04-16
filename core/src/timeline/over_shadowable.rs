use std::cmp::Ordering;

/// Interface to represent a class which can have overshadow relation between its instances.
/// In {@link VersionedIntervalTimeline}, Overshadowable is used to represent each {@link DataSegment}
/// which has the same major version in the same time chunk.
///
/// An Overshadowable overshadows another if its root partition range contains that of another
/// and has a higher minorVersion.
pub(crate) trait OverShadowable {
    fn overshadows(&self, other: &impl OverShadowable) -> bool {
        let major_version_compare = self.get_version().cmp(&other.get_version());
        if major_version_compare == Ordering::Equal {
            self.contains_root_partition(other)
                && self.get_minor_version() > other.get_minor_version()
        } else {
            major_version_compare > Ordering::Equal
        }
    }

    fn contains_root_partition(&self, other: &impl OverShadowable) -> bool {
        self.get_start_root_partition_id() <= other.get_start_root_partition_id()
            && self.get_end_root_partition_id() >= other.get_end_root_partition_id()
    }

    /// All overshadowables have root partition range.
    /// First-generation overshadowables have (partitionId, partitionId + 1) as their root partition range.
    /// Non-first-generation overshadowables are the overshadowables that overwrite first or non-first generation
    /// overshadowables, and they have the merged root partition range of all overwritten first-generation overshadowables.
    ///
    /// Note that first-generation overshadowables can be overwritten by a single non-first-generation overshadowable
    /// if they have consecutive partitionId. Non-first-generation overshadowables can be overwritten by another
    /// if their root partition ranges are consecutive.
    fn get_start_root_partition_id(&self) -> u32;

    /// See doc of {@link #get_start_root_partition_id()}.
    fn get_end_root_partition_id(&self) -> u32;

    fn get_version(&self) -> String;

    fn get_minor_version(&self) -> u16;

    /// Return the size of atomicUpdateGroup.
    /// An atomicUpdateGroup is a set of segments which should be updated all together atomically in
    /// {@link VersionedIntervalTimeline}.
    fn get_atomic_update_group_size(&self) -> u16;
}
