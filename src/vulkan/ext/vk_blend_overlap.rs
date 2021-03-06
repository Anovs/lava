// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkBlendOverlapEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBlendOverlapEXT.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkBlendOverlap {
    Uncorrelated = 0,
    Disjoint = 1,
    Conjoint = 2,
}

#[doc(hidden)]
pub type RawVkBlendOverlap = i32;

impl VkWrappedType<RawVkBlendOverlap> for VkBlendOverlap {
    fn vk_to_raw(src: &VkBlendOverlap, dst: &mut RawVkBlendOverlap) {
        *dst = *src as i32
    }
}

impl VkRawType<VkBlendOverlap> for RawVkBlendOverlap {
    fn vk_to_wrapped(src: &RawVkBlendOverlap) -> VkBlendOverlap {
        unsafe {
            *((src as *const i32) as *const VkBlendOverlap)
        }
    }
}

impl Default for VkBlendOverlap {
    fn default() -> VkBlendOverlap {
        VkBlendOverlap::Uncorrelated
    }
}