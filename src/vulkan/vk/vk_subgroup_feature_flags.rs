// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkSubgroupFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSubgroupFeatureFlagBits.html).
///
/// Use the macro `VkSubgroupFeatureFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkSubgroupFeatureFlags!(basic, vote)
/// ```
/// ```
/// VkSubgroupFeatureFlags {
///     basic: true,
///     vote: true,
///     ..VkSubgroupFeatureFlags::none()
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkSubgroupFeatureFlags {
    pub basic: bool,
    pub vote: bool,
    pub arithmetic: bool,
    pub ballot: bool,
    pub shuffle: bool,
    pub shuffle_relative: bool,
    pub clustered: bool,
    pub quad: bool,
    pub partitioned_nv: bool,
}

#[doc(hidden)]
pub type RawVkSubgroupFeatureFlags = u32;

impl VkWrappedType<RawVkSubgroupFeatureFlags> for VkSubgroupFeatureFlags {
    fn vk_to_raw(src: &VkSubgroupFeatureFlags, dst: &mut RawVkSubgroupFeatureFlags) {
        *dst = 0;
        if src.basic { *dst |= 0x00000001; }
        if src.vote { *dst |= 0x00000002; }
        if src.arithmetic { *dst |= 0x00000004; }
        if src.ballot { *dst |= 0x00000008; }
        if src.shuffle { *dst |= 0x00000010; }
        if src.shuffle_relative { *dst |= 0x00000020; }
        if src.clustered { *dst |= 0x00000040; }
        if src.quad { *dst |= 0x00000080; }
        if src.partitioned_nv { *dst |= 0x00000100; }
    }
}

impl VkRawType<VkSubgroupFeatureFlags> for RawVkSubgroupFeatureFlags {
    fn vk_to_wrapped(src: &RawVkSubgroupFeatureFlags) -> VkSubgroupFeatureFlags {
        VkSubgroupFeatureFlags {
            basic: (src & 0x00000001) != 0,
            vote: (src & 0x00000002) != 0,
            arithmetic: (src & 0x00000004) != 0,
            ballot: (src & 0x00000008) != 0,
            shuffle: (src & 0x00000010) != 0,
            shuffle_relative: (src & 0x00000020) != 0,
            clustered: (src & 0x00000040) != 0,
            quad: (src & 0x00000080) != 0,
            partitioned_nv: (src & 0x00000100) != 0,
        }
    }
}

impl Default for VkSubgroupFeatureFlags {
    fn default() -> VkSubgroupFeatureFlags {
        VkSubgroupFeatureFlags {
            basic: false,
            vote: false,
            arithmetic: false,
            ballot: false,
            shuffle: false,
            shuffle_relative: false,
            clustered: false,
            quad: false,
            partitioned_nv: false,
        }
    }
}

impl VkSubgroupFeatureFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkSubgroupFeatureFlags {
            basic: false,
            vote: false,
            arithmetic: false,
            ballot: false,
            shuffle: false,
            shuffle_relative: false,
            clustered: false,
            quad: false,
            partitioned_nv: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkSubgroupFeatureFlags {
            basic: true,
            vote: true,
            arithmetic: true,
            ballot: true,
            shuffle: true,
            shuffle_relative: true,
            clustered: true,
            quad: true,
            partitioned_nv: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.basic { 0x00000001 } else { 0 }
        + if self.vote { 0x00000002 } else { 0 }
        + if self.arithmetic { 0x00000004 } else { 0 }
        + if self.ballot { 0x00000008 } else { 0 }
        + if self.shuffle { 0x00000010 } else { 0 }
        + if self.shuffle_relative { 0x00000020 } else { 0 }
        + if self.clustered { 0x00000040 } else { 0 }
        + if self.quad { 0x00000080 } else { 0 }
        + if self.partitioned_nv { 0x00000100 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkSubgroupFeatureFlags {
            basic: value & 0x00000001 > 0,
            vote: value & 0x00000002 > 0,
            arithmetic: value & 0x00000004 > 0,
            ballot: value & 0x00000008 > 0,
            shuffle: value & 0x00000010 > 0,
            shuffle_relative: value & 0x00000020 > 0,
            clustered: value & 0x00000040 > 0,
            quad: value & 0x00000080 > 0,
            partitioned_nv: value & 0x00000100 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkSubgroupFeatureFlags {
    ( $( $x:ident ),* ) => {
        VkSubgroupFeatureFlags {
            $($x: true,)*
            ..VkSubgroupFeatureFlags::none()
        }
    }
}