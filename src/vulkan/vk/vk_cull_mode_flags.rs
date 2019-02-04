// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkCullModeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCullModeFlagBits.html).
///
/// Use the macro `VkCullModeFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkCullModeFlags!(front, back)
/// ```
/// ```
/// VkCullModeFlags {
///     front: true,
///     back: true,
///     ..VkCullModeFlags::none()
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkCullModeFlags {
    pub front: bool,
    pub back: bool,
    pub front_and_back: bool,
}

#[doc(hidden)]
pub type RawVkCullModeFlags = u32;

impl VkWrappedType<RawVkCullModeFlags> for VkCullModeFlags {
    fn vk_to_raw(src: &VkCullModeFlags, dst: &mut RawVkCullModeFlags) {
        *dst = 0;
        if src.front { *dst |= 0x00000001; }
        if src.back { *dst |= 0x00000002; }
        if src.front_and_back { *dst |= 0x00000003; }
    }
}

impl VkRawType<VkCullModeFlags> for RawVkCullModeFlags {
    fn vk_to_wrapped(src: &RawVkCullModeFlags) -> VkCullModeFlags {
        VkCullModeFlags {
            front: (src & 0x00000001) != 0,
            back: (src & 0x00000002) != 0,
            front_and_back: (src & 0x00000003) != 0,
        }
    }
}

impl Default for VkCullModeFlags {
    fn default() -> VkCullModeFlags {
        VkCullModeFlags {
            front: false,
            back: false,
            front_and_back: false,
        }
    }
}

impl VkCullModeFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkCullModeFlags {
            front: false,
            back: false,
            front_and_back: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkCullModeFlags {
            front: true,
            back: true,
            front_and_back: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.front { 0x00000001 } else { 0 }
        + if self.back { 0x00000002 } else { 0 }
        + if self.front_and_back { 0x00000003 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkCullModeFlags {
            front: value & 0x00000001 > 0,
            back: value & 0x00000002 > 0,
            front_and_back: value & 0x00000003 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkCullModeFlags {
    ( $( $x:ident ),* ) => {
        VkCullModeFlags {
            $($x: true,)*
            ..VkCullModeFlags::none()
        }
    }
}