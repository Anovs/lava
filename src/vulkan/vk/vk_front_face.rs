// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkFrontFace](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFrontFace.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkFrontFace {
    CounterClockwise = 0,
    Clockwise = 1,
}

#[doc(hidden)]
pub type RawVkFrontFace = i32;

impl VkWrappedType<RawVkFrontFace> for VkFrontFace {
    fn vk_to_raw(src: &VkFrontFace, dst: &mut RawVkFrontFace) {
        *dst = *src as i32
    }
}

impl VkRawType<VkFrontFace> for RawVkFrontFace {
    fn vk_to_wrapped(src: &RawVkFrontFace) -> VkFrontFace {
        unsafe {
            *((src as *const i32) as *const VkFrontFace)
        }
    }
}

impl Default for VkFrontFace {
    fn default() -> VkFrontFace {
        VkFrontFace::CounterClockwise
    }
}