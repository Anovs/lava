// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkDisplayPlaneAlphaFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDisplayPlaneAlphaFlagBitsKHR.html)
///
/// Use the macro `VkDisplayPlaneAlphaFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkDisplayPlaneAlphaFlags!(opaque, global)
/// ```
/// ```
/// VkDisplayPlaneAlphaFlags {
///     opaque: true,
///     global: true,
///     ..VkDisplayPlaneAlphaFlags::none()
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkDisplayPlaneAlphaFlags {
    pub opaque: bool,
    pub global: bool,
    pub per_pixel: bool,
    pub per_pixel_premultiplied: bool,
}

#[doc(hidden)]
pub type RawVkDisplayPlaneAlphaFlags = u32;

impl VkWrappedType<RawVkDisplayPlaneAlphaFlags> for VkDisplayPlaneAlphaFlags {
    fn vk_to_raw(src: &VkDisplayPlaneAlphaFlags, dst: &mut RawVkDisplayPlaneAlphaFlags) {
        *dst = 0;
        if src.opaque { *dst |= 0x00000001; }
        if src.global { *dst |= 0x00000002; }
        if src.per_pixel { *dst |= 0x00000004; }
        if src.per_pixel_premultiplied { *dst |= 0x00000008; }
    }
}

impl VkRawType<VkDisplayPlaneAlphaFlags> for RawVkDisplayPlaneAlphaFlags {
    fn vk_to_wrapped(src: &RawVkDisplayPlaneAlphaFlags) -> VkDisplayPlaneAlphaFlags {
        VkDisplayPlaneAlphaFlags {
            opaque: (src & 0x00000001) != 0,
            global: (src & 0x00000002) != 0,
            per_pixel: (src & 0x00000004) != 0,
            per_pixel_premultiplied: (src & 0x00000008) != 0,
        }
    }
}

impl Default for VkDisplayPlaneAlphaFlags {
    fn default() -> VkDisplayPlaneAlphaFlags {
        VkDisplayPlaneAlphaFlags {
            opaque: false,
            global: false,
            per_pixel: false,
            per_pixel_premultiplied: false,
        }
    }
}

impl VkDisplayPlaneAlphaFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkDisplayPlaneAlphaFlags {
            opaque: false,
            global: false,
            per_pixel: false,
            per_pixel_premultiplied: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkDisplayPlaneAlphaFlags {
            opaque: true,
            global: true,
            per_pixel: true,
            per_pixel_premultiplied: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.opaque { 0x00000001 } else { 0 }
        + if self.global { 0x00000002 } else { 0 }
        + if self.per_pixel { 0x00000004 } else { 0 }
        + if self.per_pixel_premultiplied { 0x00000008 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkDisplayPlaneAlphaFlags {
            opaque: value & 0x00000001 > 0,
            global: value & 0x00000002 > 0,
            per_pixel: value & 0x00000004 > 0,
            per_pixel_premultiplied: value & 0x00000008 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkDisplayPlaneAlphaFlags {
    ( $( $x:ident ),* ) => {
        VkDisplayPlaneAlphaFlags {
            $($x: true,)*
            ..VkDisplayPlaneAlphaFlags::none()
        }
    }
}