// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkExternalSemaphoreFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkExternalSemaphoreFeatureFlagBits.html)
///
/// Use the macro `VkExternalSemaphoreFeatureFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkExternalSemaphoreFeatureFlags!(exportable, importable)
/// ```
/// ```
/// VkExternalSemaphoreFeatureFlags {
///     exportable: true,
///     importable: true,
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkExternalSemaphoreFeatureFlags {
    pub exportable: bool,
    pub importable: bool,
}

#[doc(hidden)]
pub type RawVkExternalSemaphoreFeatureFlags = u32;

impl VkWrappedType<RawVkExternalSemaphoreFeatureFlags> for VkExternalSemaphoreFeatureFlags {
    fn vk_to_raw(src: &VkExternalSemaphoreFeatureFlags, dst: &mut RawVkExternalSemaphoreFeatureFlags) {
        *dst = 0;
        if src.exportable { *dst |= 0x00000001; }
        if src.importable { *dst |= 0x00000002; }
    }
}

impl VkRawType<VkExternalSemaphoreFeatureFlags> for RawVkExternalSemaphoreFeatureFlags {
    fn vk_to_wrapped(src: &RawVkExternalSemaphoreFeatureFlags) -> VkExternalSemaphoreFeatureFlags {
        VkExternalSemaphoreFeatureFlags {
            exportable: (src & 0x00000001) != 0,
            importable: (src & 0x00000002) != 0,
        }
    }
}

impl Default for VkExternalSemaphoreFeatureFlags {
    fn default() -> VkExternalSemaphoreFeatureFlags {
        VkExternalSemaphoreFeatureFlags {
            exportable: false,
            importable: false,
        }
    }
}

impl VkExternalSemaphoreFeatureFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkExternalSemaphoreFeatureFlags {
            exportable: false,
            importable: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkExternalSemaphoreFeatureFlags {
            exportable: true,
            importable: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.exportable { 0x00000001 } else { 0 }
        + if self.importable { 0x00000002 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkExternalSemaphoreFeatureFlags {
            exportable: value & 0x00000001 > 0,
            importable: value & 0x00000002 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkExternalSemaphoreFeatureFlags {
    ( $( $x:ident ),* ) => {
        VkExternalSemaphoreFeatureFlags {
            $($x: true,)*
            ..VkExternalSemaphoreFeatureFlags::none()
        }
    }
}