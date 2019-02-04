// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkSharingMode](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSharingMode.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkSharingMode {
    Exclusive = 0,
    Concurrent = 1,
}

#[doc(hidden)]
pub type RawVkSharingMode = i32;

impl VkWrappedType<RawVkSharingMode> for VkSharingMode {
    fn vk_to_raw(src: &VkSharingMode, dst: &mut RawVkSharingMode) {
        *dst = *src as i32
    }
}

impl VkRawType<VkSharingMode> for RawVkSharingMode {
    fn vk_to_wrapped(src: &RawVkSharingMode) -> VkSharingMode {
        unsafe {
            *((src as *const i32) as *const VkSharingMode)
        }
    }
}

impl Default for VkSharingMode {
    fn default() -> VkSharingMode {
        VkSharingMode::Exclusive
    }
}