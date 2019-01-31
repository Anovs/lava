// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPolygonMode](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPolygonMode.html)
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkPolygonMode {
    Fill = 0,
    Line = 1,
    Point = 2,
    FillRectangleNv = 1000153000,
}

#[doc(hidden)]
pub type RawVkPolygonMode = i32;

impl VkWrappedType<RawVkPolygonMode> for VkPolygonMode {
    fn vk_to_raw(src: &VkPolygonMode, dst: &mut RawVkPolygonMode) {
        *dst = *src as i32
    }
}

impl VkRawType<VkPolygonMode> for RawVkPolygonMode {
    fn vk_to_wrapped(src: &RawVkPolygonMode) -> VkPolygonMode {
        unsafe {
            *((src as *const i32) as *const VkPolygonMode)
        }
    }
}

impl Default for VkPolygonMode {
    fn default() -> VkPolygonMode {
        VkPolygonMode::Fill
    }
}