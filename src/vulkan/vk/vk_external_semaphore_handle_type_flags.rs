// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkExternalSemaphoreHandleTypeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBits.html)
///
/// Use the macro `VkExternalSemaphoreHandleTypeFlags!` as an alternative method to create a structure. For example, these two snippets return the same value:
/// ```
/// VkExternalSemaphoreHandleTypeFlags!(opaque_fd, opaque_win_32)
/// ```
/// ```
/// VkExternalSemaphoreHandleTypeFlags {
///     opaque_fd: true,
///     opaque_win_32: true,
///     ..VkExternalSemaphoreHandleTypeFlags::none()
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct VkExternalSemaphoreHandleTypeFlags {
    pub opaque_fd: bool,
    pub opaque_win_32: bool,
    pub opaque_win_32_kmt: bool,
    pub d_3d_12_fence: bool,
    pub sync_fd: bool,
}

#[doc(hidden)]
pub type RawVkExternalSemaphoreHandleTypeFlags = u32;

impl VkWrappedType<RawVkExternalSemaphoreHandleTypeFlags> for VkExternalSemaphoreHandleTypeFlags {
    fn vk_to_raw(src: &VkExternalSemaphoreHandleTypeFlags, dst: &mut RawVkExternalSemaphoreHandleTypeFlags) {
        *dst = 0;
        if src.opaque_fd { *dst |= 0x00000001; }
        if src.opaque_win_32 { *dst |= 0x00000002; }
        if src.opaque_win_32_kmt { *dst |= 0x00000004; }
        if src.d_3d_12_fence { *dst |= 0x00000008; }
        if src.sync_fd { *dst |= 0x00000010; }
    }
}

impl VkRawType<VkExternalSemaphoreHandleTypeFlags> for RawVkExternalSemaphoreHandleTypeFlags {
    fn vk_to_wrapped(src: &RawVkExternalSemaphoreHandleTypeFlags) -> VkExternalSemaphoreHandleTypeFlags {
        VkExternalSemaphoreHandleTypeFlags {
            opaque_fd: (src & 0x00000001) != 0,
            opaque_win_32: (src & 0x00000002) != 0,
            opaque_win_32_kmt: (src & 0x00000004) != 0,
            d_3d_12_fence: (src & 0x00000008) != 0,
            sync_fd: (src & 0x00000010) != 0,
        }
    }
}

impl Default for VkExternalSemaphoreHandleTypeFlags {
    fn default() -> VkExternalSemaphoreHandleTypeFlags {
        VkExternalSemaphoreHandleTypeFlags {
            opaque_fd: false,
            opaque_win_32: false,
            opaque_win_32_kmt: false,
            d_3d_12_fence: false,
            sync_fd: false,
        }
    }
}

impl VkExternalSemaphoreHandleTypeFlags {
    
    /// Return a structure with all flags to `false`.
    pub fn none() -> Self {
        VkExternalSemaphoreHandleTypeFlags {
            opaque_fd: false,
            opaque_win_32: false,
            opaque_win_32_kmt: false,
            d_3d_12_fence: false,
            sync_fd: false,
        }
    }
    
    /// Return a structure with all flags to `true`.
    pub fn all() -> Self {
        VkExternalSemaphoreHandleTypeFlags {
            opaque_fd: true,
            opaque_win_32: true,
            opaque_win_32_kmt: true,
            d_3d_12_fence: true,
            sync_fd: true,
        }
    }
    
    /// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).
    pub fn to_u32(&self) -> u32 {
        0
        + if self.opaque_fd { 0x00000001 } else { 0 }
        + if self.opaque_win_32 { 0x00000002 } else { 0 }
        + if self.opaque_win_32_kmt { 0x00000004 } else { 0 }
        + if self.d_3d_12_fence { 0x00000008 } else { 0 }
        + if self.sync_fd { 0x00000010 } else { 0 }
    }
    
    /// Create a structure corresponding to the specified numerical bit flags.
    pub fn from_u32(value: u32) -> Self {
        VkExternalSemaphoreHandleTypeFlags {
            opaque_fd: value & 0x00000001 > 0,
            opaque_win_32: value & 0x00000002 > 0,
            opaque_win_32_kmt: value & 0x00000004 > 0,
            d_3d_12_fence: value & 0x00000008 > 0,
            sync_fd: value & 0x00000010 > 0,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! VkExternalSemaphoreHandleTypeFlags {
    ( $( $x:ident ),* ) => {
        VkExternalSemaphoreHandleTypeFlags {
            $($x: true,)*
            ..VkExternalSemaphoreHandleTypeFlags::none()
        }
    }
}