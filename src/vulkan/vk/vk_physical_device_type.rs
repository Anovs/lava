// Generated by `scripts/generate.js`

use utils::vk_traits::*;

/// Wrapper for [VkPhysicalDeviceType](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceType.html).
#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkPhysicalDeviceType {
    Other = 0,
    IntegratedGpu = 1,
    DiscreteGpu = 2,
    VirtualGpu = 3,
    Cpu = 4,
}

#[doc(hidden)]
pub type RawVkPhysicalDeviceType = i32;

impl VkWrappedType<RawVkPhysicalDeviceType> for VkPhysicalDeviceType {
    fn vk_to_raw(src: &VkPhysicalDeviceType, dst: &mut RawVkPhysicalDeviceType) {
        *dst = *src as i32
    }
}

impl VkRawType<VkPhysicalDeviceType> for RawVkPhysicalDeviceType {
    fn vk_to_wrapped(src: &RawVkPhysicalDeviceType) -> VkPhysicalDeviceType {
        unsafe {
            *((src as *const i32) as *const VkPhysicalDeviceType)
        }
    }
}

impl Default for VkPhysicalDeviceType {
    fn default() -> VkPhysicalDeviceType {
        VkPhysicalDeviceType::Other
    }
}