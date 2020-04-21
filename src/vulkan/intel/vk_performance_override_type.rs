// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPerformanceOverrideTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceOverrideTypeINTEL.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkPerformanceOverrideType {
    NullHardware = 0,
    FlushGpuCaches = 1,
}

#[doc(hidden)]
pub type RawVkPerformanceOverrideType = i32;

impl VkWrappedType<RawVkPerformanceOverrideType> for VkPerformanceOverrideType {
    fn vk_to_raw(src: &VkPerformanceOverrideType, dst: &mut RawVkPerformanceOverrideType) {
        *dst = *src as i32
    }
}

impl VkRawType<VkPerformanceOverrideType> for RawVkPerformanceOverrideType {
    fn vk_to_wrapped(src: &RawVkPerformanceOverrideType) -> VkPerformanceOverrideType {
        unsafe {
            *((src as *const i32) as *const VkPerformanceOverrideType)
        }
    }
}

impl Default for VkPerformanceOverrideType {
    fn default() -> VkPerformanceOverrideType {
        VkPerformanceOverrideType::NullHardware
    }
}