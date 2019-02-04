// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPeerMemoryFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPeerMemoryFeatureFlagBits.html)
///
/// Use the macro `VkPeerMemoryFeatureFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkPeerMemoryFeatureFlags!(copy_src, copy_dst)
/// ```
/// ```
/// VkPeerMemoryFeatureFlags {
///     copy_src: true,
///     copy_dst: true,
///     ..VkPeerMemoryFeatureFlags::none()
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkPeerMemoryFeatureFlags {
    pub copy_src: bool,
    pub copy_dst: bool,
    pub generic_src: bool,
    pub generic_dst: bool,
}

#[doc(hidden)]
pub type RawVkPeerMemoryFeatureFlags = u32;

impl VkWrappedType<RawVkPeerMemoryFeatureFlags> for VkPeerMemoryFeatureFlags {
    fn vk_to_raw(src: &VkPeerMemoryFeatureFlags, dst: &mut RawVkPeerMemoryFeatureFlags) {
        *dst = 0;
        if src.copy_src { *dst |= 0x00000001; }
        if src.copy_dst { *dst |= 0x00000002; }
        if src.generic_src { *dst |= 0x00000004; }
        if src.generic_dst { *dst |= 0x00000008; }
    }
}

impl VkRawType<VkPeerMemoryFeatureFlags> for RawVkPeerMemoryFeatureFlags {
    fn vk_to_wrapped(src: &RawVkPeerMemoryFeatureFlags) -> VkPeerMemoryFeatureFlags {
        VkPeerMemoryFeatureFlags {
            copy_src: (src & 0x00000001) != 0,
            copy_dst: (src & 0x00000002) != 0,
            generic_src: (src & 0x00000004) != 0,
            generic_dst: (src & 0x00000008) != 0,
        }
    }
}

impl Default for VkPeerMemoryFeatureFlags {
    fn default() -> VkPeerMemoryFeatureFlags {
        VkPeerMemoryFeatureFlags {
            copy_src: false,
            copy_dst: false,
            generic_src: false,
            generic_dst: false,
        }
    }
}

impl VkPeerMemoryFeatureFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkPeerMemoryFeatureFlags {
            copy_src: false,
            copy_dst: false,
            generic_src: false,
            generic_dst: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkPeerMemoryFeatureFlags {
            copy_src: true,
            copy_dst: true,
            generic_src: true,
            generic_dst: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.copy_src { 0x00000001 } else { 0 }
        + if self.copy_dst { 0x00000002 } else { 0 }
        + if self.generic_src { 0x00000004 } else { 0 }
        + if self.generic_dst { 0x00000008 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkPeerMemoryFeatureFlags {
            copy_src: value & 0x00000001 > 0,
            copy_dst: value & 0x00000002 > 0,
            generic_src: value & 0x00000004 > 0,
            generic_dst: value & 0x00000008 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkPeerMemoryFeatureFlags {
    ( $( $x:ident ),* ) => {
        VkPeerMemoryFeatureFlags {
            $($x: true,)*
            ..VkPeerMemoryFeatureFlags::none()
        }
    }
}