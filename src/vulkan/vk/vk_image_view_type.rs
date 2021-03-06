// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkImageViewType](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewType.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkImageViewType {
    _1d = 0,
    _2d = 1,
    _3d = 2,
    Cube = 3,
    _1dArray = 4,
    _2dArray = 5,
    CubeArray = 6,
}

#[doc(hidden)]
pub type RawVkImageViewType = i32;

impl VkWrappedType<RawVkImageViewType> for VkImageViewType {
    fn vk_to_raw(src: &VkImageViewType, dst: &mut RawVkImageViewType) {
        *dst = *src as i32
    }
}

impl VkRawType<VkImageViewType> for RawVkImageViewType {
    fn vk_to_wrapped(src: &RawVkImageViewType) -> VkImageViewType {
        unsafe {
            *((src as *const i32) as *const VkImageViewType)
        }
    }
}

impl Default for VkImageViewType {
    fn default() -> VkImageViewType {
        VkImageViewType::_1d
    }
}