// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkCommandBufferLevel](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandBufferLevel.html)
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkCommandBufferLevel {
    Primary = 0,
    Secondary = 1,
}

#[doc(hidden)]
pub type RawVkCommandBufferLevel = i32;

impl VkWrappedType<RawVkCommandBufferLevel> for VkCommandBufferLevel {
    fn vk_to_raw(src: &VkCommandBufferLevel, dst: &mut RawVkCommandBufferLevel) {
        *dst = *src as i32
    }
}

impl VkRawType<VkCommandBufferLevel> for RawVkCommandBufferLevel {
    fn vk_to_wrapped(src: &RawVkCommandBufferLevel) -> VkCommandBufferLevel {
        unsafe {
            *((src as *const i32) as *const VkCommandBufferLevel)
        }
    }
}

impl Default for VkCommandBufferLevel {
    fn default() -> VkCommandBufferLevel {
        VkCommandBufferLevel::Primary
    }
}