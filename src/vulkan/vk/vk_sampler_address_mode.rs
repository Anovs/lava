// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkSamplerAddressMode](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerAddressMode.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkSamplerAddressMode {
    Repeat = 0,
    MirroredRepeat = 1,
    ClampToEdge = 2,
    ClampToBorder = 3,
    MirrorClampToEdge = 4,
}

#[doc(hidden)]
pub type RawVkSamplerAddressMode = i32;

impl VkWrappedType<RawVkSamplerAddressMode> for VkSamplerAddressMode {
    fn vk_to_raw(src: &VkSamplerAddressMode, dst: &mut RawVkSamplerAddressMode) {
        *dst = *src as i32
    }
}

impl VkRawType<VkSamplerAddressMode> for RawVkSamplerAddressMode {
    fn vk_to_wrapped(src: &RawVkSamplerAddressMode) -> VkSamplerAddressMode {
        unsafe {
            *((src as *const i32) as *const VkSamplerAddressMode)
        }
    }
}

impl Default for VkSamplerAddressMode {
    fn default() -> VkSamplerAddressMode {
        VkSamplerAddressMode::Repeat
    }
}