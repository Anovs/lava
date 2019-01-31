// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkSurfaceTransformFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html)
#[derive(Debug, Clone, Copy)]
pub struct VkSurfaceTransformFlags {
    pub identity: bool,
    pub rotate_90: bool,
    pub rotate_180: bool,
    pub rotate_270: bool,
    pub horizontal_mirror: bool,
    pub horizontal_mirror_rotate_90: bool,
    pub horizontal_mirror_rotate_180: bool,
    pub horizontal_mirror_rotate_270: bool,
    pub inherit: bool,
}

#[doc(hidden)]
pub type RawVkSurfaceTransformFlags = u32;

impl VkWrappedType<RawVkSurfaceTransformFlags> for VkSurfaceTransformFlags {
    fn vk_to_raw(src: &VkSurfaceTransformFlags, dst: &mut RawVkSurfaceTransformFlags) {
        *dst = 0;
        if src.identity { *dst |= 0x00000001; }
        if src.rotate_90 { *dst |= 0x00000002; }
        if src.rotate_180 { *dst |= 0x00000004; }
        if src.rotate_270 { *dst |= 0x00000008; }
        if src.horizontal_mirror { *dst |= 0x00000010; }
        if src.horizontal_mirror_rotate_90 { *dst |= 0x00000020; }
        if src.horizontal_mirror_rotate_180 { *dst |= 0x00000040; }
        if src.horizontal_mirror_rotate_270 { *dst |= 0x00000080; }
        if src.inherit { *dst |= 0x00000100; }
    }
}

impl VkRawType<VkSurfaceTransformFlags> for RawVkSurfaceTransformFlags {
    fn vk_to_wrapped(src: &RawVkSurfaceTransformFlags) -> VkSurfaceTransformFlags {
        VkSurfaceTransformFlags {
            identity: (src & 0x00000001) != 0,
            rotate_90: (src & 0x00000002) != 0,
            rotate_180: (src & 0x00000004) != 0,
            rotate_270: (src & 0x00000008) != 0,
            horizontal_mirror: (src & 0x00000010) != 0,
            horizontal_mirror_rotate_90: (src & 0x00000020) != 0,
            horizontal_mirror_rotate_180: (src & 0x00000040) != 0,
            horizontal_mirror_rotate_270: (src & 0x00000080) != 0,
            inherit: (src & 0x00000100) != 0,
        }
    }
}

impl Default for VkSurfaceTransformFlags {
    fn default() -> VkSurfaceTransformFlags {
        VkSurfaceTransformFlags {
            identity: false,
            rotate_90: false,
            rotate_180: false,
            rotate_270: false,
            horizontal_mirror: false,
            horizontal_mirror_rotate_90: false,
            horizontal_mirror_rotate_180: false,
            horizontal_mirror_rotate_270: false,
            inherit: false,
        }
    }
}

impl VkSurfaceTransformFlags {
    
    pub fn none() -> VkSurfaceTransformFlags {
        VkSurfaceTransformFlags {
            identity: false,
            rotate_90: false,
            rotate_180: false,
            rotate_270: false,
            horizontal_mirror: false,
            horizontal_mirror_rotate_90: false,
            horizontal_mirror_rotate_180: false,
            horizontal_mirror_rotate_270: false,
            inherit: false,
        }
    }
    
    pub fn all() -> VkSurfaceTransformFlags {
        VkSurfaceTransformFlags {
            identity: true,
            rotate_90: true,
            rotate_180: true,
            rotate_270: true,
            horizontal_mirror: true,
            horizontal_mirror_rotate_90: true,
            horizontal_mirror_rotate_180: true,
            horizontal_mirror_rotate_270: true,
            inherit: true,
        }
    }
}

#[macro_export]
macro_rules! VkSurfaceTransformFlags {
    ( $( $x:ident ),* ) => {
        VkSurfaceTransformFlags {
            $($x: true,)*
            ..VkSurfaceTransformFlags::none()
        }
    }
}

impl VkSurfaceTransformFlags {
    
    pub fn to_u32(&self) -> u32 {
        0
        + if self.identity { 0x00000001 } else { 0 }
        + if self.rotate_90 { 0x00000002 } else { 0 }
        + if self.rotate_180 { 0x00000004 } else { 0 }
        + if self.rotate_270 { 0x00000008 } else { 0 }
        + if self.horizontal_mirror { 0x00000010 } else { 0 }
        + if self.horizontal_mirror_rotate_90 { 0x00000020 } else { 0 }
        + if self.horizontal_mirror_rotate_180 { 0x00000040 } else { 0 }
        + if self.horizontal_mirror_rotate_270 { 0x00000080 } else { 0 }
        + if self.inherit { 0x00000100 } else { 0 }
    }
}