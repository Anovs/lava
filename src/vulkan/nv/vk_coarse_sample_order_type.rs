// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkCoarseSampleOrderTypeNV](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCoarseSampleOrderTypeNV.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkCoarseSampleOrderType {
    Default = 0,
    Custom = 1,
    PixelMajor = 2,
    SampleMajor = 3,
}

#[doc(hidden)]
pub type RawVkCoarseSampleOrderType = i32;

impl VkWrappedType<RawVkCoarseSampleOrderType> for VkCoarseSampleOrderType {
    fn vk_to_raw(src: &VkCoarseSampleOrderType, dst: &mut RawVkCoarseSampleOrderType) {
        *dst = *src as i32
    }
}

impl VkRawType<VkCoarseSampleOrderType> for RawVkCoarseSampleOrderType {
    fn vk_to_wrapped(src: &RawVkCoarseSampleOrderType) -> VkCoarseSampleOrderType {
        unsafe {
            *((src as *const i32) as *const VkCoarseSampleOrderType)
        }
    }
}

impl Default for VkCoarseSampleOrderType {
    fn default() -> VkCoarseSampleOrderType {
        VkCoarseSampleOrderType::Default
    }
}