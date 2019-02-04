// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkBuildAccelerationStructureFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBuildAccelerationStructureFlagBitsNV.html).
///
/// Use the macro `VkBuildAccelerationStructureFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkBuildAccelerationStructureFlags!(allow_update, allow_compaction)
/// ```
/// ```
/// VkBuildAccelerationStructureFlags {
///     allow_update: true,
///     allow_compaction: true,
///     ..VkBuildAccelerationStructureFlags::none()
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkBuildAccelerationStructureFlags {
    pub allow_update: bool,
    pub allow_compaction: bool,
    pub prefer_fast_trace: bool,
    pub prefer_fast_build: bool,
    pub low_memory: bool,
}

#[doc(hidden)]
pub type RawVkBuildAccelerationStructureFlags = u32;

impl VkWrappedType<RawVkBuildAccelerationStructureFlags> for VkBuildAccelerationStructureFlags {
    fn vk_to_raw(src: &VkBuildAccelerationStructureFlags, dst: &mut RawVkBuildAccelerationStructureFlags) {
        *dst = 0;
        if src.allow_update { *dst |= 0x00000001; }
        if src.allow_compaction { *dst |= 0x00000002; }
        if src.prefer_fast_trace { *dst |= 0x00000004; }
        if src.prefer_fast_build { *dst |= 0x00000008; }
        if src.low_memory { *dst |= 0x00000010; }
    }
}

impl VkRawType<VkBuildAccelerationStructureFlags> for RawVkBuildAccelerationStructureFlags {
    fn vk_to_wrapped(src: &RawVkBuildAccelerationStructureFlags) -> VkBuildAccelerationStructureFlags {
        VkBuildAccelerationStructureFlags {
            allow_update: (src & 0x00000001) != 0,
            allow_compaction: (src & 0x00000002) != 0,
            prefer_fast_trace: (src & 0x00000004) != 0,
            prefer_fast_build: (src & 0x00000008) != 0,
            low_memory: (src & 0x00000010) != 0,
        }
    }
}

impl Default for VkBuildAccelerationStructureFlags {
    fn default() -> VkBuildAccelerationStructureFlags {
        VkBuildAccelerationStructureFlags {
            allow_update: false,
            allow_compaction: false,
            prefer_fast_trace: false,
            prefer_fast_build: false,
            low_memory: false,
        }
    }
}

impl VkBuildAccelerationStructureFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkBuildAccelerationStructureFlags {
            allow_update: false,
            allow_compaction: false,
            prefer_fast_trace: false,
            prefer_fast_build: false,
            low_memory: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkBuildAccelerationStructureFlags {
            allow_update: true,
            allow_compaction: true,
            prefer_fast_trace: true,
            prefer_fast_build: true,
            low_memory: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.allow_update { 0x00000001 } else { 0 }
        + if self.allow_compaction { 0x00000002 } else { 0 }
        + if self.prefer_fast_trace { 0x00000004 } else { 0 }
        + if self.prefer_fast_build { 0x00000008 } else { 0 }
        + if self.low_memory { 0x00000010 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkBuildAccelerationStructureFlags {
            allow_update: value & 0x00000001 > 0,
            allow_compaction: value & 0x00000002 > 0,
            prefer_fast_trace: value & 0x00000004 > 0,
            prefer_fast_build: value & 0x00000008 > 0,
            low_memory: value & 0x00000010 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkBuildAccelerationStructureFlags {
    ( $( $x:ident ),* ) => {
        VkBuildAccelerationStructureFlags {
            $($x: true,)*
            ..VkBuildAccelerationStructureFlags::none()
        }
    }
}