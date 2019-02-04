// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkDependencyFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDependencyFlagBits.html).
///
/// Use the macro `VkDependencyFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkDependencyFlags!(by_region, device_group)
/// ```
/// ```
/// VkDependencyFlags {
///     by_region: true,
///     device_group: true,
///     ..VkDependencyFlags::none()
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkDependencyFlags {
    pub by_region: bool,
    pub device_group: bool,
    pub view_local: bool,
}

#[doc(hidden)]
pub type RawVkDependencyFlags = u32;

impl VkWrappedType<RawVkDependencyFlags> for VkDependencyFlags {
    fn vk_to_raw(src: &VkDependencyFlags, dst: &mut RawVkDependencyFlags) {
        *dst = 0;
        if src.by_region { *dst |= 0x00000001; }
        if src.device_group { *dst |= 0x00000004; }
        if src.view_local { *dst |= 0x00000002; }
    }
}

impl VkRawType<VkDependencyFlags> for RawVkDependencyFlags {
    fn vk_to_wrapped(src: &RawVkDependencyFlags) -> VkDependencyFlags {
        VkDependencyFlags {
            by_region: (src & 0x00000001) != 0,
            device_group: (src & 0x00000004) != 0,
            view_local: (src & 0x00000002) != 0,
        }
    }
}

impl Default for VkDependencyFlags {
    fn default() -> VkDependencyFlags {
        VkDependencyFlags {
            by_region: false,
            device_group: false,
            view_local: false,
        }
    }
}

impl VkDependencyFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkDependencyFlags {
            by_region: false,
            device_group: false,
            view_local: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkDependencyFlags {
            by_region: true,
            device_group: true,
            view_local: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.by_region { 0x00000001 } else { 0 }
        + if self.device_group { 0x00000004 } else { 0 }
        + if self.view_local { 0x00000002 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkDependencyFlags {
            by_region: value & 0x00000001 > 0,
            device_group: value & 0x00000004 > 0,
            view_local: value & 0x00000002 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkDependencyFlags {
    ( $( $x:ident ),* ) => {
        VkDependencyFlags {
            $($x: true,)*
            ..VkDependencyFlags::none()
        }
    }
}