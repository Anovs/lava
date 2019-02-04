// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkStencilFaceFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkStencilFaceFlagBits.html).
///
/// Use the macro `VkStencilFaceFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkStencilFaceFlags!(front, back)
/// ```
/// ```
/// VkStencilFaceFlags {
///     front: true,
///     back: true,
///     ..VkStencilFaceFlags::none()
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkStencilFaceFlags {
    pub front: bool,
    pub back: bool,
    pub _and_back: bool,
}

#[doc(hidden)]
pub type RawVkStencilFaceFlags = u32;

impl VkWrappedType<RawVkStencilFaceFlags> for VkStencilFaceFlags {
    fn vk_to_raw(src: &VkStencilFaceFlags, dst: &mut RawVkStencilFaceFlags) {
        *dst = 0;
        if src.front { *dst |= 0x00000001; }
        if src.back { *dst |= 0x00000002; }
        if src._and_back { *dst |= 0x00000003; }
    }
}

impl VkRawType<VkStencilFaceFlags> for RawVkStencilFaceFlags {
    fn vk_to_wrapped(src: &RawVkStencilFaceFlags) -> VkStencilFaceFlags {
        VkStencilFaceFlags {
            front: (src & 0x00000001) != 0,
            back: (src & 0x00000002) != 0,
            _and_back: (src & 0x00000003) != 0,
        }
    }
}

impl Default for VkStencilFaceFlags {
    fn default() -> VkStencilFaceFlags {
        VkStencilFaceFlags {
            front: false,
            back: false,
            _and_back: false,
        }
    }
}

impl VkStencilFaceFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkStencilFaceFlags {
            front: false,
            back: false,
            _and_back: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkStencilFaceFlags {
            front: true,
            back: true,
            _and_back: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.front { 0x00000001 } else { 0 }
        + if self.back { 0x00000002 } else { 0 }
        + if self._and_back { 0x00000003 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkStencilFaceFlags {
            front: value & 0x00000001 > 0,
            back: value & 0x00000002 > 0,
            _and_back: value & 0x00000003 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkStencilFaceFlags {
    ( $( $x:ident ),* ) => {
        VkStencilFaceFlags {
            $($x: true,)*
            ..VkStencilFaceFlags::none()
        }
    }
}