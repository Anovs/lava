// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkDisplayEventTypeEXT](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDisplayEventTypeEXT.html)
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkDisplayEventType {
    FirstPixelOut = 0,
}

#[doc(hidden)]
pub type RawVkDisplayEventType = i32;

impl VkWrappedType<RawVkDisplayEventType> for VkDisplayEventType {
    fn vk_to_raw(src: &VkDisplayEventType, dst: &mut RawVkDisplayEventType) {
        *dst = *src as i32
    }
}

impl VkRawType<VkDisplayEventType> for RawVkDisplayEventType {
    fn vk_to_wrapped(src: &RawVkDisplayEventType) -> VkDisplayEventType {
        unsafe {
            *((src as *const i32) as *const VkDisplayEventType)
        }
    }
}

impl Default for VkDisplayEventType {
    fn default() -> VkDisplayEventType {
        VkDisplayEventType::FirstPixelOut
    }
}