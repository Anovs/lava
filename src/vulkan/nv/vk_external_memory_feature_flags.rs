// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkExternalMemoryFeatureFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkExternalMemoryFeatureFlagBitsNV.html).
///
/// Use the macro `VkExternalMemoryFeatureFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkExternalMemoryFeatureFlags!(dedicated_only, exportable)
/// ```
/// ```
/// VkExternalMemoryFeatureFlags {
///     dedicated_only: true,
///     exportable: true,
///     ..VkExternalMemoryFeatureFlags::none()
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkExternalMemoryFeatureFlags {
    pub dedicated_only: bool,
    pub exportable: bool,
    pub importable: bool,
}

#[doc(hidden)]
pub type RawVkExternalMemoryFeatureFlags = u32;

impl VkWrappedType<RawVkExternalMemoryFeatureFlags> for VkExternalMemoryFeatureFlags {
    fn vk_to_raw(src: &VkExternalMemoryFeatureFlags, dst: &mut RawVkExternalMemoryFeatureFlags) {
        *dst = 0;
        if src.dedicated_only { *dst |= 0x00000001; }
        if src.exportable { *dst |= 0x00000002; }
        if src.importable { *dst |= 0x00000004; }
    }
}

impl VkRawType<VkExternalMemoryFeatureFlags> for RawVkExternalMemoryFeatureFlags {
    fn vk_to_wrapped(src: &RawVkExternalMemoryFeatureFlags) -> VkExternalMemoryFeatureFlags {
        VkExternalMemoryFeatureFlags {
            dedicated_only: (src & 0x00000001) != 0,
            exportable: (src & 0x00000002) != 0,
            importable: (src & 0x00000004) != 0,
        }
    }
}

impl Default for VkExternalMemoryFeatureFlags {
    fn default() -> VkExternalMemoryFeatureFlags {
        VkExternalMemoryFeatureFlags {
            dedicated_only: false,
            exportable: false,
            importable: false,
        }
    }
}

impl VkExternalMemoryFeatureFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkExternalMemoryFeatureFlags {
            dedicated_only: false,
            exportable: false,
            importable: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkExternalMemoryFeatureFlags {
            dedicated_only: true,
            exportable: true,
            importable: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.dedicated_only { 0x00000001 } else { 0 }
        + if self.exportable { 0x00000002 } else { 0 }
        + if self.importable { 0x00000004 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkExternalMemoryFeatureFlags {
            dedicated_only: value & 0x00000001 > 0,
            exportable: value & 0x00000002 > 0,
            importable: value & 0x00000004 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkExternalMemoryFeatureFlagsNv {
    ( $( $x:ident ),* ) => {
        VkExternalMemoryFeatureFlags {
            $($x: true,)*
            ..VkExternalMemoryFeatureFlags::none()
        }
    }
}